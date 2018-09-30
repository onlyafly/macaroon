/* Primitives are build-in functions */

use ast::{Node, PrimitiveObj, Val};
use back::env::{Env, SmartEnv};
use back::runtime_error::{check_args, RuntimeError};
use loc::Loc;
use std::cell::RefMut;
use std::io;
use std::io::Write;

pub fn init_env_with_primitives(env: &SmartEnv) -> Result<(), RuntimeError> {
    let mut menv = env.borrow_mut();

    menv.define("true", Node::new(Val::Boolean(true), Loc::Unknown))?;
    menv.define("false", Node::new(Val::Boolean(false), Loc::Unknown))?;
    menv.define("nil", Node::new(Val::Nil, Loc::Unknown))?;

    define_primitive(&mut menv, "+", 2, 2)?; // TODO: should be 0, -1
    define_primitive(&mut menv, "-", 2, 2)?; // TODO: should be 1, -1 ???
    define_primitive(&mut menv, "=", 2, 2)?; // TODO: should be 2, -1
    define_primitive(&mut menv, "<", 2, 2)?;
    define_primitive(&mut menv, ">", 2, 2)?;
    define_primitive(&mut menv, "println", 0, -1)?;
    define_primitive(&mut menv, "not", 1, 1)?;

    Ok(())
}

fn define_primitive(
    mut_env: &mut RefMut<Env>,
    name: &'static str,
    min_arity: isize,
    max_arity: isize,
) -> Result<(), RuntimeError> {
    mut_env.define(
        name,
        Node::new(
            Val::Primitive(PrimitiveObj {
                name: name.to_string(),
                min_arity,
                max_arity,
            }),
            Loc::Unknown,
        ),
    )
}

pub fn eval_primitive(
    primitive_obj: PrimitiveObj,
    env: SmartEnv,
    mut args: Vec<Node>,
    loc: Loc,
) -> Result<Node, RuntimeError> {
    check_args(
        &primitive_obj.name,
        &loc,
        &args,
        primitive_obj.min_arity,
        primitive_obj.max_arity,
    )?;

    let primitive_fn = match primitive_obj.name.as_ref() {
        "+" => eval_primitive_add,
        "-" => eval_primitive_subtract,
        "=" => eval_primitive_equal,
        "<" => eval_primitive_less_than,
        ">" => eval_primitive_greater_than,
        "not" => eval_primitive_not,
        "println" => eval_primitive_println,
        _ => {
            return Err(RuntimeError::UndefinedPrimitive(
                primitive_obj.name,
                args.remove(0).loc,
            ))
        }
    };

    primitive_fn(env, args)
}

fn eval_primitive_not(_env: SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let one = args.remove(0);

    let one_bool = one.as_host_boolean()?;

    let output = !one_bool;

    let result = Node::new(Val::Boolean(output), one.loc);
    Ok(result)
}

fn eval_primitive_add(_env: SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let one = args.remove(0);
    let two = args.remove(0);

    let one_i32 = one.as_host_number()?;
    let two_i32 = two.as_host_number()?;

    let output = one_i32 + two_i32;

    let result = Node::new(Val::Number(output), one.loc);
    Ok(result)
}

fn eval_primitive_subtract(_env: SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let one = args.remove(0);
    let two = args.remove(0);

    let one_i32 = one.as_host_number()?;
    let two_i32 = two.as_host_number()?;

    let output = one_i32 - two_i32;

    let result = Node::new(Val::Number(output), one.loc);
    Ok(result)
}

fn eval_primitive_equal(_env: SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let a = args.remove(0);
    let b = args.remove(0);

    let output = a.val == b.val;

    Ok(Node::new(Val::Boolean(output), a.loc))
}

fn eval_primitive_less_than(_env: SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let a = args.remove(0);
    let b = args.remove(0);

    let output = a.val < b.val;

    Ok(Node::new(Val::Boolean(output), a.loc))
}

fn eval_primitive_greater_than(_env: SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let a = args.remove(0);
    let b = args.remove(0);

    let output = a.val > b.val;

    Ok(Node::new(Val::Boolean(output), a.loc))
}

fn eval_primitive_println(_env: SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let mut w = io::stdout();

    while args.len() > 0 {
        let n = args.remove(0);
        if let Err(_) = write!(&mut w, "{}", n.val) {
            return Err(RuntimeError::Unknown("println error".to_string(), n.loc));
        }
    }

    if let Err(_) = write!(&mut w, "\n") {
        return Err(RuntimeError::Unknown(
            "println error".to_string(),
            Loc::Unknown,
        ));
    }

    Ok(Node::new(Val::Nil, Loc::Unknown))
}
