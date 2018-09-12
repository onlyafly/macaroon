/* Primitives are build-in functions */

use ast::{Node, Value};
use back::env::Env;
use loc::Loc;

pub fn init_env_with_primitives(env: &mut Env) {
    env.insert(
        "true".to_string(),
        Node::new(Value::Boolean(true), Loc::Unknown),
    );
    env.insert(
        "false".to_string(),
        Node::new(Value::Boolean(false), Loc::Unknown),
    );
}
