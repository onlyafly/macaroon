mod env;
mod primitives;
mod specials;

use self::env::Env;
use ast::{Node, WrappedNode};
use loc::Loc;

#[allow(dead_code)]
pub enum RuntimeError {
    Rich(String, Loc),
    Simple(String),
}

pub fn create_root_env() -> Env {
    let env = Env::new();
    primitives::init_env_with_primitives(&env);
    env
}

pub fn eval(env: &mut Env, nodes: Vec<WrappedNode>) -> Result<WrappedNode, RuntimeError> {
    let mut output = WrappedNode::new(Node::Error("NO-INPUT".to_string()), Loc::empty()); // TODO: should this be nil?

    for node in nodes {
        output = eval_node(env, node)?;
    }

    Ok(output)
}

fn eval_node(env: &mut Env, wrapped_node: WrappedNode) -> Result<WrappedNode, RuntimeError> {
    let loc = wrapped_node.loc;
    match wrapped_node.node {
        Node::List { children } => eval_list(env, children),
        Node::Symbol(name) => match env.get(&name) {
            Some(&ref wrapped_node) => Ok(WrappedNode::new(wrapped_node.node.clone(), loc)),
            None => Err(RuntimeError::Simple(format!("Undefined name: {}", name))),
        },
        n @ Node::Number(_) => Ok(WrappedNode::new(n, loc)),
        n => Err(RuntimeError::Simple(format!(
            "Unable to eval node: {}",
            n.display()
        ))),
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
            _ => Err(RuntimeError::Rich(
                format!("Don't know what to do with list starting with: {}", name),
                loc,
            )),
        },
        n => {
            let evaluated_head = eval_node(env, WrappedNode::new(n, loc.clone()))?;

            match evaluated_head.node {
                Node::Proc { mut body, .. } => {
                    Ok(body.remove(0)) // TODO: we currently just return the first item in the body
                }
                _ => Err(RuntimeError::Rich(
                    format!(
                        "Don't know what to do with list starting with: {}",
                        evaluated_head.display()
                    ),
                    loc,
                )),
            }
        }
    }
}
