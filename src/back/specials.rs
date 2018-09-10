use ast::{Node, Value};
use back::env::Env;
use back::eval;
use back::runtime_error::RuntimeError;
use loc::Loc;

pub fn eval_special_list(env: &mut Env, loc: Loc, args: Vec<Node>) -> Result<Node, RuntimeError> {
    let mut evaled_args = Vec::new();

    for child in args {
        let evaled_child = eval::eval_value(env, child)?;
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

pub fn eval_special_def(env: &mut Env, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let name_node = args.remove(0);

    if let Value::Symbol(name) = name_node.value {
        if env.exists(&name) {
            return Err(RuntimeError::CannotRedefine(name, name_node.loc));
        }

        let value_node = eval::eval_value(env, args.remove(0))?;

        env.insert(name, value_node);
        Ok(Node::new(Value::Number(0), name_node.loc)) // TODO: should be nil
    } else {
        Err(RuntimeError::UnexpectedValue(
            "symbol".to_string(),
            name_node.value,
            name_node.loc,
        ))
    }
}

pub fn eval_special_update(env: &mut Env, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let name_node = args.remove(0);
    let value = name_node.value;
    let loc = name_node.loc;

    if let Value::Symbol(name) = value {
        if !env.exists(&name) {
            return Err(RuntimeError::CannotUpdateUndefinedName(name, loc));
        }
        let value = args.remove(0);
        env.insert(name, value);
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
    env: &mut Env,
    mut args: Vec<Node>,
) -> Result<Node, RuntimeError> {
    let name_node = args.remove(0);
    let value = name_node.value;
    let loc = name_node.loc;

    if let Value::Symbol(name) = value {
        if !env.exists(&name) {
            return Err(RuntimeError::CannotUpdateUndefinedName(name, loc));
        }

        let _index_value = args.remove(0);
        let value_value = eval::eval_value(env, args.remove(0))?;

        if let Some(entry) = env.remove(&name) {
            match entry.value {
                Value::List { mut children } => {
                    //TODO: get num from index_value instead of using zero
                    children[0] = value_value;
                    env.insert(name, Node::new(Value::List { children }, loc.clone()));
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
            value,
            loc,
        ))
    }
}

pub fn eval_special_fn(_env: &mut Env, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
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
