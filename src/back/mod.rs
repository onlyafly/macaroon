pub mod env;
mod eval;
mod primitives;
pub mod runtime_error;
mod specials;

use ast::{Node, Value};
use back::env::Env;
use back::runtime_error::RuntimeError;
use loc::Loc;

pub fn create_root_env() -> Result<Env, RuntimeError> {
    let mut env = Env::new();
    primitives::init_env_with_primitives(&mut env)?;
    Ok(env)
}

pub fn eval(env: &mut Env, values: Vec<Node>) -> Result<Node, RuntimeError> {
    let mut output = Node::new(Value::Error("NO-INPUT".to_string()), Loc::Unknown); // TODO: should this be nil?

    for value in values {
        output = eval::eval_node(env, value)?;
    }

    Ok(output)
}
