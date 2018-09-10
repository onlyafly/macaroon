mod env;
mod eval;
mod primitives;
mod runtime_error;
mod specials;

use ast::{Node, WrappedNode};
use back::env::Env;
use back::runtime_error::RuntimeError;
use loc::Loc;

pub fn create_root_env() -> Env {
    let env = Env::new();
    primitives::init_env_with_primitives(&env);
    env
}

pub fn eval(env: &mut Env, nodes: Vec<WrappedNode>) -> Result<WrappedNode, RuntimeError> {
    let mut output = WrappedNode::new(Node::Error("NO-INPUT".to_string()), Loc::Unknown); // TODO: should this be nil?

    for node in nodes {
        output = eval::eval_node(env, node)?;
    }

    Ok(output)
}
