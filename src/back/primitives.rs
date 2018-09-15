/* Primitives are build-in functions */

use ast::{Node, Value};
use back::env::Env;
use back::runtime_error::RuntimeError;
use loc::Loc;

pub fn init_env_with_primitives(env: &mut Env) -> Result<(), RuntimeError> {
    env.define("true", Node::new(Value::Boolean(true), Loc::Unknown))?;
    env.define("false", Node::new(Value::Boolean(false), Loc::Unknown))?;
    Ok(())
}
