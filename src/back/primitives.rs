/* Primitives are build-in functions */

use ast::{Node, Value};
use back::env::{Env, SmartEnv};
use back::runtime_error::RuntimeError;
use loc::Loc;
use std::cell::RefMut;

pub fn init_env_with_primitives(env: &SmartEnv) -> Result<(), RuntimeError> {
    let mut menv = env.borrow_mut();

    menv.define("true", Node::new(Value::Boolean(true), Loc::Unknown))?;
    menv.define("false", Node::new(Value::Boolean(false), Loc::Unknown))?;
    menv.define("nil", Node::new(Value::Nil, Loc::Unknown))?;

    define_primitive(&mut menv, "+")?;
    define_primitive(&mut menv, "-")?;
    define_primitive(&mut menv, "=")?;
    define_primitive(&mut menv, "not")?;

    Ok(())
}

fn define_primitive(mut_env: &mut RefMut<Env>, name: &'static str) -> Result<(), RuntimeError> {
    mut_env.define(
        name,
        Node::new(
            Value::Primitive {
                primitive_name: name.to_string(),
            },
            Loc::Unknown,
        ),
    )
}

pub fn eval_primitive_by_name(
    primitive_name: String,
    env: &SmartEnv,
    mut args: Vec<Node>,
) -> Result<Node, RuntimeError> {
    let primitive_fn = match primitive_name.as_ref() {
        "+" => eval_primitive_add,
        "-" => eval_primitive_subtract,
        "=" => eval_primitive_equal,
        "not" => eval_primitive_not,
        _ => {
            return Err(RuntimeError::UndefinedPrimitive(
                primitive_name,
                args.remove(0).loc,
            ))
        }
    };

    primitive_fn(env, args)
}

fn eval_primitive_not(_env: &SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let one = args.remove(0);

    let one_bool = one.as_host_boolean()?;

    let output = !one_bool;

    let result = Node::new(Value::Boolean(output), one.loc);
    Ok(result)
}

fn eval_primitive_add(_env: &SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let one = args.remove(0);
    let two = args.remove(0);

    let one_i32 = one.as_host_number()?;
    let two_i32 = two.as_host_number()?;

    let output = one_i32 + two_i32;

    let result = Node::new(Value::Number(output), one.loc);
    Ok(result)
}

fn eval_primitive_subtract(_env: &SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let one = args.remove(0);
    let two = args.remove(0);

    let one_i32 = one.as_host_number()?;
    let two_i32 = two.as_host_number()?;

    let output = one_i32 - two_i32;

    let result = Node::new(Value::Number(output), one.loc);
    Ok(result)
}

fn eval_primitive_equal(_env: &SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let a = args.remove(0);
    let b = args.remove(0);

    let output = a.value == b.value;

    Ok(Node::new(Value::Boolean(output), a.loc))
}
