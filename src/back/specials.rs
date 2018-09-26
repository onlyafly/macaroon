use ast::{Node, Value};
use back::env::{Env, SmartEnv};
use back::eval;
use back::runtime_error::RuntimeError;
use back::trampoline;
use back::trampoline::ContinuationResult;
use loc::Loc;
use std::rc::Rc;

pub fn eval_special_list(env: SmartEnv, loc: Loc, args: Vec<Node>) -> ContinuationResult {
    let mut evaled_args = Vec::new();

    for child in args {
        let evaled_child = trampoline::run(eval::eval_node, Rc::clone(&env), child)?;
        evaled_args.push(evaled_child);
    }

    Ok(trampoline::finish(Node::new(
        Value::List {
            children: evaled_args,
        },
        loc,
    )))
}

pub fn eval_special_quote(mut args: Vec<Node>) -> ContinuationResult {
    Ok(trampoline::finish(args.remove(0)))
}

pub fn eval_special_def(env: SmartEnv, mut args: Vec<Node>) -> ContinuationResult {
    let name_node = args.remove(0);

    if let Value::Symbol(name) = name_node.value {
        let value_node = trampoline::run(eval::eval_node, Rc::clone(&env), args.remove(0))?;
        env.borrow_mut().define(&name, value_node)?;
        Ok(trampoline::finish(Node::new(Value::Nil, name_node.loc))) // TODO: should be nil
    } else {
        Err(RuntimeError::UnexpectedValue(
            "symbol".to_string(),
            name_node.value,
            name_node.loc,
        ))
    }
}

pub fn eval_special_let(env: SmartEnv, mut args: Vec<Node>) -> ContinuationResult {
    let bindings_node = args.remove(0);

    match bindings_node.value {
        Value::List {
            children: mut bindings_vec,
        } => {
            let bindings_env = Env::new(Some(Rc::clone(&env)));

            while bindings_vec.len() > 1 {
                let name_node = bindings_vec.remove(0);
                let value_node =
                    trampoline::run(eval::eval_node, Rc::clone(&env), bindings_vec.remove(0))?;

                let name = match name_node.value {
                    Value::Symbol(name_node) => name_node,
                    _ => "".to_string(), // TODO: probably should throw error here
                };

                bindings_env.borrow_mut().define(&name, value_node)?;
            }

            Ok(trampoline::bounce(
                eval::eval_node,
                bindings_env,
                args.remove(0),
            ))
        }
        _ => Err(RuntimeError::UnexpectedValue(
            "let".to_string(),
            bindings_node.value,
            bindings_node.loc,
        )),
    }
}

pub fn eval_special_update(env: SmartEnv, mut args: Vec<Node>) -> ContinuationResult {
    let name_node = args.remove(0);
    let value = name_node.value;
    let loc = name_node.loc;

    if let Value::Symbol(name) = value {
        let value = trampoline::run(eval::eval_node, Rc::clone(&env), args.remove(0))?;
        env.borrow_mut().update(&name, value)?;
        Ok(trampoline::finish(Node::new(Value::Nil, loc)))
    } else {
        Err(RuntimeError::UnexpectedValue(
            "symbol".to_string(),
            value,
            loc,
        ))
    }
}

pub fn eval_special_update_element(env: SmartEnv, mut args: Vec<Node>) -> ContinuationResult {
    let name_node = args.remove(0);
    let loc = name_node.loc;

    if let Value::Symbol(name) = name_node.value {
        let mut index_node = trampoline::run(eval::eval_node, Rc::clone(&env), args.remove(0))?;
        let index = index_node.value.as_host_number()? as usize;

        let newval_node = trampoline::run(eval::eval_node, Rc::clone(&env), args.remove(0))?;

        let mut mutable_env = env.borrow_mut();

        if let Some(entry) = mutable_env.remove(&name) {
            match entry.value {
                Value::List { mut children } => {
                    //TODO: get num from index_value instead of using zero

                    if index >= children.len() {
                        return Err(RuntimeError::IndexOutOfBounds {
                            index: index,
                            len: children.len(),
                            loc: loc,
                        });
                    }

                    children[index] = newval_node;
                    mutable_env.update(&name, Node::new(Value::List { children }, loc.clone()))?;
                }
                _ => {
                    return Err(RuntimeError::CannotUpdateElementInValue(entry.value, loc));
                }
            }
        }

        Ok(trampoline::finish(Node::new(Value::Nil, loc)))
    } else {
        Err(RuntimeError::UnexpectedValue(
            "symbol".to_string(),
            name_node.value,
            loc,
        ))
    }
}

pub fn eval_special_if(env: SmartEnv, mut args: Vec<Node>) -> ContinuationResult {
    let predicate = trampoline::run(eval::eval_node, Rc::clone(&env), args.remove(0))?;
    let true_branch = args.remove(0);
    let false_branch = args.remove(0);

    let branch = match predicate.as_host_boolean()? {
        true => true_branch,
        false => false_branch,
    };

    Ok(trampoline::bounce(eval::eval_node, env, branch))
}

pub fn eval_special_fn(lexical_env: SmartEnv, mut args: Vec<Node>) -> ContinuationResult {
    let param_list = args.remove(0);
    let body = args.remove(0); // TODO: note that the body is only one node currently

    match param_list.value {
        Value::List { children } => Ok(trampoline::finish(Node::new(
            Value::Function {
                params: children,
                body: Box::new(body),
                lexical_env: Rc::clone(&lexical_env),
            },
            param_list.loc,
        ))),
        _ => Err(RuntimeError::UnexpectedValue(
            "list of parameters".to_string(),
            param_list.value,
            param_list.loc,
        )),
    }
}
