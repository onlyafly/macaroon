use ast::*;
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
    use ast::Val::*;
    match node {
        Node {
            val: List { .. }, ..
        } => Ok(trampoline::bounce(eval_list, env, node)),
        Node {
            val: Symbol(name),
            loc,
        } => match env.borrow_mut().get(&name) {
            Some(node) => Ok(trampoline::finish(node)),
            None => Err(RuntimeError::UndefinedName(name, loc)),
        },
        Node {
            val: Number(..), ..
        } => Ok(trampoline::finish(node)),
        Node {
            val: Character(..), ..
        } => Ok(trampoline::finish(node)),
        Node {
            val: StringVal(..), ..
        } => Ok(trampoline::finish(node)),
        _ => Err(RuntimeError::UnableToEvalValue(node.val, node.loc)),
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

pub fn eval_each_node_for_single_output(
    env: SmartEnv,
    nodes: Vec<Node>,
) -> Result<Node, RuntimeError> {
    let mut output = Node::new(Val::Nil, Loc::Unknown);
    for node in nodes {
        output = trampoline::run(eval_node, Rc::clone(&env), node)?;
    }
    Ok(output)
}

fn eval_list(env: SmartEnv, node: Node, _: Vec<Node>) -> ContinuationResult {
    let loc = node.loc;
    let mut args = match node.val {
        Val::List { children } => children,
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
                return specials::eval_special_fn(env, args);
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

    let evaled_head = trampoline::run(
        eval_node,
        Rc::clone(&env),
        Node::new(head_value, loc.clone()),
    )?;

    match evaled_head.val {
        Val::Function { .. } => Ok(trampoline::bounce_with_nodes(
            eval_invoke_proc,
            Rc::clone(&env),
            evaled_head,
            args,
        )),
        Val::Primitive(obj) => {
            let out = eval_invoke_primitive(obj, Rc::clone(&env), args, loc)?;
            Ok(trampoline::finish(out))
        }
        _ => Err(RuntimeError::UnableToEvalListStartingWith(
            format!("{}", evaled_head.val),
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
    match proc.val {
        Val::Function {
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

                match param.val {
                    Val::Symbol(name) => {
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
