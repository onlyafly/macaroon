pub mod env;
mod eval;
mod primitives;
pub mod runtime_error;
mod specials;
mod trampoline;

use ast::{Node, Value};
use back::env::{Env, SmartEnv};
use back::runtime_error::RuntimeError;
use loc::Loc;
use std::rc::Rc;

pub fn create_root_env() -> Result<SmartEnv, RuntimeError> {
    let env = Env::new(None);
    primitives::init_env_with_primitives(&env)?;
    Ok(env)
}

pub fn eval(env: SmartEnv, values: Vec<Node>) -> Result<Node, RuntimeError> {
    let mut output = Node::new(Value::Error("NO-INPUT".to_string()), Loc::Unknown); // TODO: should this be nil?

    for value in values {
        output = trampoline::run(eval::eval_node, Rc::clone(&env), value)?;
    }

    Ok(output)
}
