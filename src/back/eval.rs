use ast::*;
use back::env::{Env, SmartEnv};
use back::primitives::eval_primitive;
use back::runtime_error::{check_args, RuntimeError};
use back::specials;
use back::trampoline;
use back::trampoline::{ContinuationResult, Flag};
use loc::Loc;
use std::rc::Rc;

pub type NodeResult = Result<Node, RuntimeError>;

pub fn eval_node(env: SmartEnv, node: Node, _: Vec<Node>, _: Flag) -> ContinuationResult {
    match node.val {
        Val::List(..) => Ok(trampoline::bounce(eval_list, env, node)),
        Val::Symbol(name) => match env.borrow_mut().get(&name) {
            Some(node) => Ok(trampoline::finish(node)),
            None => Err(RuntimeError::UndefinedName(name, node.loc)),
        },
        _ => Ok(trampoline::finish(node)),
    }
}

fn eval_each_node(env: SmartEnv, nodes: Vec<Node>) -> Result<Vec<Node>, RuntimeError> {
    let mut outputs = Vec::new();
    for node in nodes {
        let output = trampoline::run(eval_node, Rc::clone(&env), node)?;
        outputs.push(output);
    }
    Ok(outputs)
}

pub fn eval_each_node_for_single_output(env: SmartEnv, nodes: Vec<Node>) -> NodeResult {
    let mut output = Node::new(Val::Nil, Loc::Unknown);
    for node in nodes {
        output = trampoline::run(eval_node, Rc::clone(&env), node)?;
    }
    Ok(output)
}

pub fn eval_list(env: SmartEnv, node: Node, _: Vec<Node>, flag: Flag) -> ContinuationResult {
    let loc = node.loc;
    let mut args = match node.val {
        Val::List(children) => children,
        _ => panic!("expected list"),
    };

    if args.len() == 0 {
        return Err(RuntimeError::CannotEvalEmptyList(loc));
    }

    let head_node = args.remove(0);
    let head_value = head_node.val;

    match head_value {
        Val::Symbol(ref name) => match name.as_ref() {
            "def" => {
                check_args("def", &loc, &args, 2, 2)?;
                return specials::eval_special_def(env, args);
            }
            "quote" => {
                check_args("quote", &loc, &args, 1, -1)?;
                return specials::eval_special_quote(args);
            }
            "list" => {
                check_args("list", &loc, &args, 0, -1)?;
                return specials::eval_special_list(env, loc, args);
            }
            "fn" => {
                check_args("fn", &loc, &args, 2, 2)?;
                return specials::eval_special_routine(env, args, RoutineType::Function);
            }
            "macro" => {
                check_args("macro", &loc, &args, 2, 2)?;
                return specials::eval_special_routine(env, args, RoutineType::Macro);
            }
            "macroexpand1" => {
                check_args("macroexpand1", &loc, &args, 1, 1)?;
                return specials::eval_special_macroexpand1(env, args);
            }
            "if" => {
                check_args("if", &loc, &args, 3, 3)?;
                return specials::eval_special_if(env, args);
            }
            "cond" => {
                check_args("cond", &loc, &args, 2, -1)?;
                return specials::eval_special_cond(env, args);
            }
            "for" => {
                check_args("for", &loc, &args, 4, 4)?;
                return specials::eval_special_for(env, args);
            }
            "let" => {
                check_args("let", &loc, &args, 2, -1)?;
                return specials::eval_special_let(env, args);
            }
            "update!" => {
                check_args("update!", &loc, &args, 2, 2)?;
                return specials::eval_special_update(env, args);
            }
            "update-element!" => {
                check_args("update-element!", &loc, &args, 3, 3)?;
                return specials::eval_special_update_element(env, args);
            }
            "begin" => {
                check_args("begin", &loc, &args, 0, -1)?;
                return specials::eval_special_begin(env, args);
            }
            _ => {}
        },
        _ => {}
    }

    let mut evaled_head = trampoline::run(
        eval_node,
        Rc::clone(&env),
        Node::new(head_value, loc.clone()),
    )?;

    // Sometimes the evaled head will lack a location. When that happens, the location needs
    // to be set to the location of the unevaled head, to allow for good error messages.
    if evaled_head.loc == Loc::Unknown {
        evaled_head.loc = loc.clone();
    }

    match evaled_head.val {
        Val::Routine(..) | Val::Primitive(..) => {
            eval_invoke_procedure(env, evaled_head, args, flag)
        }
        _ => Err(RuntimeError::UnableToEvalListStartingWith(
            format!("{}", evaled_head.val),
            loc,
        )),
    }
}

pub fn eval_invoke_procedure(
    env: SmartEnv,
    head: Node,
    args: Vec<Node>,
    flag: Flag,
) -> ContinuationResult {
    match head.val {
        Val::Routine(..) => Ok(trampoline::bounce_with_nodes(
            eval_invoke_routine,
            Rc::clone(&env),
            head,
            args,
            flag,
        )),
        Val::Primitive(obj) => {
            let out = eval_invoke_primitive(obj, Rc::clone(&env), args, head.loc)?;
            Ok(trampoline::finish(out))
        }
        _ => Err(RuntimeError::CannotInvokeNonProcedure(
            head.val.to_string(),
            head.loc,
        )),
    }
}

fn eval_invoke_primitive(
    obj: PrimitiveObj,
    dynamic_env: SmartEnv,
    unevaled_args: Vec<Node>,
    loc: Loc,
) -> NodeResult {
    let evaled_args = eval_each_node(Rc::clone(&dynamic_env), unevaled_args)?;
    eval_primitive(obj, dynamic_env, evaled_args, loc)
}

pub fn eval_invoke_routine(
    dynamic_env: SmartEnv,
    fnode: Node,
    unevaled_args: Vec<Node>,
    flag: Flag,
) -> ContinuationResult {
    let loc = fnode.loc;

    if let Val::Routine(robj) = fnode.val {
        let mut params = robj.params;
        let body = robj.body;
        let parent_lexical_env = robj.lexical_env;

        // Determine if there is a &rest param
        let mut has_variable_params = false;
        for param in &params {
            if param.val == Val::Symbol("&rest".to_string()) {
                has_variable_params = true;
                break;
            }
        }

        if !has_variable_params && (params.len() != unevaled_args.len()) {
            // The args and params don't match
            return Err(RuntimeError::FunctionArgsDoNotMatchParams {
                function_name: robj.name,
                params_count: params.len(),
                args_count: unevaled_args.len(),
                params_list: params,
                args_list: unevaled_args,
                loc,
            });
        }

        // Create the lexical environment based on the procedure's lexical parent
        let lexical_env = Env::new(Some(parent_lexical_env));

        // Prepare the arguments for evaluation
        let mut args = match robj.routine_type {
            RoutineType::Macro => unevaled_args,
            RoutineType::Function => eval_each_node(Rc::clone(&dynamic_env), unevaled_args)?,
        };

        // Map arguments to parameters
        while params.len() > 0 {
            let param = params.remove(0);

            match param.val {
                Val::Symbol(ref name) if name == "&rest" => {
                    if params.len() != 1 {
                        return Err(RuntimeError::TooManyFunctionParamsAfterRest {
                            function_name: robj.name,
                            remaining_params: params,
                            loc,
                        });
                    }

                    let rest_param = params.remove(0);
                    match rest_param.val {
                        Val::Symbol(name) => {
                            let l = Node::new(Val::List(args), rest_param.loc);
                            lexical_env.borrow_mut().define(&name, l)?;
                            break;
                        }
                        v => return Err(RuntimeError::ParamsMustBeSymbols(v, loc)),
                    }
                }
                Val::Symbol(name) => {
                    let arg = if args.len() > 0 {
                        args.remove(0)
                    } else {
                        return Err(RuntimeError::Unknown("not enough args".to_string(), loc));
                    };

                    lexical_env.borrow_mut().define(&name, arg)?;
                }
                v => return Err(RuntimeError::ParamsMustBeSymbols(v, loc)),
            }
        }

        // Evaluate the application of the routine
        match robj.routine_type {
            RoutineType::Macro => {
                let expanded_macro = trampoline::run(eval_node, lexical_env, *body)?;

                match flag {
                    Flag::None => {
                        // This is executed in the environment of its application, not the
                        // environment of its definition
                        return Ok(trampoline::bounce(eval_node, dynamic_env, expanded_macro));
                    }
                    Flag::DelayMacroEvaluation => {
                        return Ok(trampoline::finish(expanded_macro));
                    }
                }
            }
            RoutineType::Function => {
                return Ok(trampoline::bounce(eval_node, lexical_env, *body));
            }
        }
    }

    return Err(RuntimeError::CannotInvokeNonProcedure(
        fnode.val.to_string(),
        loc,
    ));
}
