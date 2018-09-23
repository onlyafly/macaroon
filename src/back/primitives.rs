/* Primitives are build-in functions */

use ast::{Node, Value};
#[allow(unused_imports)]
use back::env::{Env, SmartEnv};
use back::runtime_error::RuntimeError;
use loc::Loc;

pub fn init_env_with_primitives(env: &SmartEnv) -> Result<(), RuntimeError> {
    let mut menv = env.borrow_mut();

    menv.define("true", Node::new(Value::Boolean(true), Loc::Unknown))?;
    menv.define("false", Node::new(Value::Boolean(false), Loc::Unknown))?;
    menv.define(
        "+",
        Node::new(
            Value::Primitive {
                primitive_name: "+".to_string(),
            },
            Loc::Unknown,
        ),
    )?;

    Ok(())
}

pub fn eval_primitive_by_name(
    primitive_name: String,
    env: &SmartEnv,
    mut args: Vec<Node>,
) -> Result<Node, RuntimeError> {
    let primitive_fn = match primitive_name.as_ref() {
        "+" => eval_primitive_addition,
        _ => panic!("Unknown primitive function"),
    };

    primitive_fn(env, args)
}

fn eval_primitive_addition(_env: &SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let one = args.remove(0);
    let two = args.remove(0);

    let one_i32 = one.as_host_number()?;
    let two_i32 = two.as_host_number()?;

    let addition_result = one_i32 + two_i32;

    let result = Node::new(Value::Number(addition_result), one.loc);
    Ok(result)
}
