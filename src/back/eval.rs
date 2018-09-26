use ast::{Node, PrimitiveObj, Value};
use back::env::{Env, SmartEnv};
use back::primitives::eval_primitive;
use back::runtime_error::{check_args, RuntimeError};
use back::specials;
use back::trampoline;
use back::trampoline::ContinuationResult;
use loc::Loc;
use std::rc::Rc;

type EvalResult = Result<Node, RuntimeError>;

pub fn eval_node(env: SmartEnv, node: Node, _: Vec<Node>) -> ContinuationResult {
    let loc = node.loc;
    match node.value {
        v @ Value::List { .. } => Ok(trampoline::bounce(eval_list, env, Node::new(v, loc))),
        Value::Symbol(name) => match env.borrow_mut().get(&name) {
            Some(node) => Ok(trampoline::finish(node)),
            None => Err(RuntimeError::UndefinedName(name, loc)),
        },
        n @ Value::Number(_) => Ok(trampoline::finish(Node::new(n, loc))),
        n => Err(RuntimeError::UnableToEvalValue(n, loc)),
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

fn eval_list(env: SmartEnv, node: Node, _: Vec<Node>) -> ContinuationResult {
    let loc = node.loc;
    let mut args = match node.value {
        Value::List { children } => children,
        _ => panic!("expected list"),
    };

    if args.len() == 0 {
        return Err(RuntimeError::CannotEvalEmptyList(loc));
    }

    let head_node = args.remove(0);
    let head_value = head_node.value;

    match head_value {
        Value::Symbol(ref name) => match name.as_ref() {
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
                return specials::eval_special_fn(env, args);
            }
            "if" => {
                check_args("if", &loc, &args, 3, 3)?;
                return specials::eval_special_if(env, args);
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
            _ => {}
        },
        _ => {}
    }

    let evaled_head = trampoline::run(
        eval_node,
        Rc::clone(&env),
        Node::new(head_value, loc.clone()),
    )?;

    match evaled_head.value {
        Value::Function { .. } => Ok(trampoline::bounce_with_nodes(
            eval_invoke_proc,
            Rc::clone(&env),
            evaled_head,
            args,
        )),
        Value::Primitive(obj) => {
            let out = eval_invoke_primitive(obj, Rc::clone(&env), args, loc)?;
            Ok(trampoline::finish(out))
        }
        _ => Err(RuntimeError::UnableToEvalListStartingWith(
            evaled_head.display(),
            loc,
        )),
    }
}

fn eval_invoke_primitive(
    obj: PrimitiveObj,
    dynamic_env: SmartEnv,
    unevaled_args: Vec<Node>,
    loc: Loc,
) -> EvalResult {
    let evaled_args = eval_each_node(Rc::clone(&dynamic_env), unevaled_args)?;
    eval_primitive(obj, dynamic_env, evaled_args, loc)
}

fn eval_invoke_proc(
    dynamic_env: SmartEnv,
    proc: Node,
    unevaled_args: Vec<Node>,
) -> ContinuationResult {
    let loc = proc.loc;
    match proc.value {
        Value::Function {
            params,
            body,
            lexical_env: parent_lexical_env,
        } => {
            // Validate params
            if unevaled_args.len() != params.len() {
                return Err(RuntimeError::ProcArgsDoNotMatchParams(String::new(), loc));
            }

            // Create the lexical environment based on the procedure's lexical parent
            let lexical_env = Env::new(Some(parent_lexical_env));

            // Prepare the arguments for evaluation
            let mut evaled_args = eval_each_node(dynamic_env, unevaled_args)?;

            // Map arguments to parameters
            for param in params {
                let evaled_arg = match evaled_args.pop() {
                    None => return Err(RuntimeError::Unknown("not enough args".to_string(), loc)),
                    Some(n) => n,
                };

                match param.value {
                    Value::Symbol(name) => {
                        lexical_env.borrow_mut().define(&name, evaled_arg)?;
                    }
                    _ => return Err(RuntimeError::Unknown("param not a symbol".to_string(), loc)),
                }
            }

            // Evaluate the application of the procedure
            Ok(trampoline::bounce(eval_node, lexical_env, *body))
        }
        _ => panic!("Cannot invoke a non-procedure"),
    }
}
