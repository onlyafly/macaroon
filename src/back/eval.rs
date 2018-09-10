use ast::{Node, WrappedNode};
use back::env::Env;
use back::runtime_error::RuntimeError;
use back::specials;

pub fn eval_node(env: &mut Env, wrapped_node: WrappedNode) -> Result<WrappedNode, RuntimeError> {
    let loc = wrapped_node.loc;
    match wrapped_node.node {
        Node::List { children } => eval_list(env, children),
        Node::Symbol(name) => match env.get(&name) {
            Some(&ref wrapped_node) => Ok(WrappedNode::new(wrapped_node.node.clone(), loc)),
            None => Err(RuntimeError::UndefinedName(name, loc)),
        },
        n @ Node::Number(_) => Ok(WrappedNode::new(n, loc)),
        n => Err(RuntimeError::UnableToEvalNode(n, loc)),
    }
}

fn eval_list(env: &mut Env, mut children: Vec<WrappedNode>) -> Result<WrappedNode, RuntimeError> {
    let wrapped_node = children.remove(0);
    let node = wrapped_node.node;
    let loc = wrapped_node.loc;

    match node {
        Node::Symbol(ref name) => match name.as_ref() {
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
            let evaluated_head = eval_node(env, WrappedNode::new(n, loc.clone()))?;

            match evaluated_head.node {
                Node::Proc { mut body, .. } => {
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
