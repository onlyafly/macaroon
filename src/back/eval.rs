use ast::{Node, Value};
use back::env::Env;
use back::runtime_error::RuntimeError;
use back::specials;

pub fn eval_value(env: &mut Env, node: Node) -> Result<Node, RuntimeError> {
    let loc = node.loc;
    match node.value {
        Value::List { children } => eval_list(env, children),
        Value::Symbol(name) => match env.get(&name) {
            Some(&ref node) => Ok(Node::new(node.value.clone(), loc)),
            None => Err(RuntimeError::UndefinedName(name, loc)),
        },
        n @ Value::Number(_) => Ok(Node::new(n, loc)),
        n => Err(RuntimeError::UnableToEvalValue(n, loc)),
    }
}

fn eval_list(env: &mut Env, mut children: Vec<Node>) -> Result<Node, RuntimeError> {
    let node = children.remove(0);
    let value = node.value;
    let loc = node.loc;

    match value {
        Value::Symbol(ref name) => match name.as_ref() {
            "list" => specials::eval_special_list(env, loc, children),
            "quote" => specials::eval_special_quote(children),
            "def" => specials::eval_special_def(env, children),
            "fn" => specials::eval_special_fn(env, children),
            "update!" => specials::eval_special_update(env, children),
            "update-element!" => specials::eval_special_update_element(env, children),
            _ => Err(RuntimeError::UnableToEvalListStartingWith(
                name.clone(),
                loc,
            )),
        },
        n => {
            let evaluated_head = eval_value(env, Node::new(n, loc.clone()))?;

            match evaluated_head.value {
                Value::Proc { mut body, .. } => {
                    Ok(body.remove(0)) // TODO: we currently just return the first item in the body
                }
                _ => Err(RuntimeError::UnableToEvalListStartingWith(
                    evaluated_head.display(),
                    loc,
                )),
            }
        }
    }
}
