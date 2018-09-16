/* Primitives are build-in functions */

use ast::{Node, Value};
#[allow(unused_imports)]
use back::env::{Env, SmartEnv};
use back::runtime_error::RuntimeError;
use loc::Loc;

pub fn init_env_with_primitives(env: &SmartEnv) -> Result<(), RuntimeError> {
    env.borrow_mut()
        .define("true", Node::new(Value::Boolean(true), Loc::Unknown))?;
    env.borrow_mut()
        .define("false", Node::new(Value::Boolean(false), Loc::Unknown))?;
    Ok(())
}
