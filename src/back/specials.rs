use ast::{Node, Value};
#[allow(unused_imports)]
use back::env::{Env, SmartEnv};
use back::eval;
use back::runtime_error::RuntimeError;
use loc::Loc;
use std::rc::Rc;

pub fn eval_special_list(env: &SmartEnv, loc: Loc, args: Vec<Node>) -> Result<Node, RuntimeError> {
    let mut evaled_args = Vec::new();

    for child in args {
        let evaled_child = eval::eval_node(env, child)?;
        evaled_args.push(evaled_child);
    }

    Ok(Node::new(
        Value::List {
            children: evaled_args,
        },
        loc,
    ))
}

pub fn eval_special_quote(mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    Ok(args.remove(0))
}

pub fn eval_special_def(env: &SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let name_node = args.remove(0);

    if let Value::Symbol(name) = name_node.value {
        let value_node = eval::eval_node(env, args.remove(0))?;
        env.borrow_mut().define(&name, value_node)?;
        Ok(Node::new(Value::Number(0), name_node.loc)) // TODO: should be nil
    } else {
        Err(RuntimeError::UnexpectedValue(
            "symbol".to_string(),
            name_node.value,
            name_node.loc,
        ))
    }
}

pub fn eval_special_let(env: &SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let bindings_node = args.remove(0);

    match bindings_node.value {
        Value::List {
            children: mut bindings_vec,
        } => {
            let bindings_env = Env::new(Some(Rc::clone(env)));

            let mut index = 0;
            while index < bindings_vec.len() {
                let name_node = bindings_vec.remove(0);
                let value_node = eval::eval_node(env, bindings_vec.remove(0))?;

                let name = match name_node.value {
                    Value::Symbol(name_node) => name_node,
                    _ => "".to_string(), // TODO: probably should throw error here
                };

                bindings_env.borrow_mut().define(&name, value_node)?;

                index += 2;
            }

            let body_node = eval::eval_node(&bindings_env, args.remove(0))?;
            Ok(body_node)
        }
        _ => Err(RuntimeError::UnexpectedValue(
            "let".to_string(),
            bindings_node.value,
            bindings_node.loc,
        )),
    }
}

pub fn eval_special_update(env: &SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let name_node = args.remove(0);
    let value = name_node.value;
    let loc = name_node.loc;

    if let Value::Symbol(name) = value {
        let value = eval::eval_node(env, args.remove(0))?;
        env.borrow_mut().update(&name, value)?;
        Ok(Node::new(Value::Number(0), loc)) // TODO: should be nil
    } else {
        Err(RuntimeError::UnexpectedValue(
            "symbol".to_string(),
            value,
            loc,
        ))
    }
}

pub fn eval_special_update_element(
    env: &SmartEnv,
    mut args: Vec<Node>,
) -> Result<Node, RuntimeError> {
    let name_node = args.remove(0);
    let loc = name_node.loc;

    if let Value::Symbol(name) = name_node.value {
        let mut index_node = eval::eval_node(env, args.remove(0))?;
        let index = index_node.value.as_number_value()? as usize;

        let newval_node = eval::eval_node(env, args.remove(0))?;

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

        Ok(Node::new(Value::Number(0), loc)) // TODO: should be nil
    } else {
        Err(RuntimeError::UnexpectedValue(
            "symbol".to_string(),
            name_node.value,
            loc,
        ))
    }
}

pub fn eval_special_if(env: &SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let predicate = eval::eval_node(env, args.remove(0))?;
    let true_branch = args.remove(0);
    let false_branch = args.remove(0);

    let branch = match predicate.as_boolean_value()? {
        true => true_branch,
        false => false_branch,
    };

    let result = eval::eval_node(env, branch)?;
    Ok(result)
}

pub fn eval_special_fn(_env: &SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let param_list = args.remove(0);
    let body = args;

    match param_list.value {
        Value::List { children } => Ok(Node::new(
            Value::Proc {
                params: children,
                body: body,
            },
            param_list.loc,
        )),
        _ => Err(RuntimeError::UnexpectedValue(
            "list of parameters".to_string(),
            param_list.value,
            param_list.loc,
        )),
    }
}
