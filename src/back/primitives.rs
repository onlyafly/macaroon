/* Primitives are build-in functions */

use ast::{Node, PrimitiveObj, ReaderObj, Val, WriterObj};
use back::env::{Env, SmartEnv};
use back::eval;
use back::runtime_error::{check_args, RuntimeError};
use back::trampoline;
use loc::Loc;
use std::cell::RefMut;
use std::fs::File;
use std::io::prelude::*;

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

    define_primitive(&mut menv, "panic", 0, -1)?;
    define_primitive(&mut menv, "read-line", 0, 0)?;
    define_primitive(&mut menv, "println", 0, -1)?;
    define_primitive(&mut menv, "not", 1, 1)?;
    define_primitive(&mut menv, "apply", 2, 2)?;
    define_primitive(&mut menv, "typeof", 1, 1)?;
    define_primitive(&mut menv, "load", 1, 1)?;

    define_primitive(&mut menv, "str", 0, -1)?;
    define_primitive(&mut menv, "concat", 0, -1)?;
    define_primitive(&mut menv, "cons", 2, 2)?;
    define_primitive(&mut menv, "first", 1, 1)?;
    define_primitive(&mut menv, "rest", 1, 1)?;
    define_primitive(&mut menv, "len", 1, 1)?;

    define_primitive(&mut menv, "current-environment", 0, 0)?;
    define_primitive(&mut menv, "eval", 1, 1)?;

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
        "panic" => eval_primitive_panic,
        "read-line" => eval_primitive_read_line,
        "println" => eval_primitive_println,
        "apply" => eval_primitive_apply,
        "typeof" => eval_primitive_typeof,
        "load" => eval_primitive_load,

        "str" => eval_primitive_str,
        "concat" => eval_primitive_concat,
        "cons" => eval_primitive_cons,
        "first" => eval_primitive_first,
        "rest" => eval_primitive_rest,
        "len" => eval_primitive_len,

        "current-environment" => eval_primitive_current_environment,
        "eval" => eval_primitive_eval,

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

fn eval_primitive_panic(_env: SmartEnv, args: Vec<Node>) -> Result<Node, RuntimeError> {
    let mut v = Vec::new();
    let mut loc = Loc::Unknown;
    for arg in args {
        loc = arg.loc.clone();
        v.push(arg.as_print_friendly_string());
    }
    let output = format!("{}\n", &v.join(" "));

    Err(RuntimeError::ApplicationPanic(output, loc))
}

fn eval_primitive_read_line(env: SmartEnv, _args: Vec<Node>) -> Result<Node, RuntimeError> {
    match env.borrow().get("*reader*") {
        Some(node) => match node.val {
            Val::Reader(ReaderObj { reader_function }) => match reader_function() {
                Ok(output) => Ok(Node::new(Val::StringVal(output), Loc::Unknown)),
                Err(s) => Err(RuntimeError::Unknown(
                    format!("Problem while reading: {}", s),
                    Loc::Unknown,
                )),
            },
            v => Err(RuntimeError::UnexpectedValue(
                "reader".to_string(),
                v,
                Loc::Unknown,
            )),
        },
        _ => panic!("expected reader value"),
    }
}

fn eval_primitive_println(env: SmartEnv, args: Vec<Node>) -> Result<Node, RuntimeError> {
    let mut v = Vec::new();
    let mut loc = Loc::Unknown;
    for arg in args {
        loc = arg.loc.clone();
        v.push(arg.as_print_friendly_string());
    }
    let output = format!("{}\n", &v.join(" "));

    match env.borrow().get("*writer*") {
        Some(node) => match node.val {
            Val::Writer(WriterObj::Standard) => {
                print!("{}", output);
            }
            Val::Writer(WriterObj::Buffer(b)) => {
                use std::io::Write;
                let mut rm_buffer = b.borrow_mut();
                write!(&mut rm_buffer, "{}", output).expect("unable to write to buffer");
            }
            v => return Err(RuntimeError::UnexpectedValue("writer".to_string(), v, loc)),
        },
        _ => panic!("expected writer value"),
    }

    Ok(Node::new(Val::Nil, loc))
}

fn eval_primitive_apply(env: SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let f = args.remove(0);
    let f_params_node = args.remove(0);
    let f_params = match f_params_node.val {
        Val::List { children } => children,
        v => {
            return Err(RuntimeError::UnexpectedValue(
                "list".to_string(),
                v,
                f_params_node.loc,
            ))
        }
    };

    let output = trampoline::run_with_nodes(eval::eval_invoke_procedure, env, f, f_params)?;

    Ok(output)
}

fn eval_primitive_typeof(_env: SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let arg = args.remove(0);

    let output = arg.type_name()?;

    Ok(Node::new(Val::Symbol(output), arg.loc))
}

fn eval_primitive_load(env: SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let filename_node = args.remove(0);

    let filename = match filename_node.val {
        Val::StringVal(s) => s,
        v => {
            return Err(RuntimeError::UnexpectedValue(
                "file name".to_string(),
                v,
                filename_node.loc,
            ))
        }
    };

    let mut f = File::open(filename.to_string()).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let output = ::parse_eval_print(env, &filename, &contents);
    println!("{}", output); // TODO: This doesn't actually deal with any errors while loading right now

    Ok(Node::new(Val::Nil, filename_node.loc))
}

fn eval_primitive_str(_env: SmartEnv, args: Vec<Node>) -> Result<Node, RuntimeError> {
    let mut v = Vec::new();
    let mut loc = Loc::Unknown;
    for arg in args {
        loc = arg.loc.clone();
        v.push(arg.as_print_friendly_string());
    }
    let output = format!("{}", &v.join(""));

    Ok(Node::new(Val::StringVal(output), loc))
}

fn eval_primitive_concat(_env: SmartEnv, args: Vec<Node>) -> Result<Node, RuntimeError> {
    let mut output = Node::new(Val::Nil, Loc::Unknown);
    for mut arg in args {
        output = output.coll_append(arg)?;
    }

    Ok(output)
}

fn eval_primitive_cons(_env: SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let elem = args.remove(0);
    let coll = args.remove(0);

    let output = coll.coll_cons(elem)?;

    Ok(output)
}

fn eval_primitive_first(_env: SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    Ok(args.remove(0).coll_first()?)
}

fn eval_primitive_rest(_env: SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    Ok(args.remove(0).coll_rest()?)
}

fn eval_primitive_len(_env: SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let n = args.remove(0);
    let loc = n.loc.clone();

    let out = n.coll_len()?;

    Ok(Node::new(Val::Number(out as i32), loc))
}

fn eval_primitive_current_environment(
    env: SmartEnv,
    _args: Vec<Node>,
) -> Result<Node, RuntimeError> {
    Ok(Node::new(Val::Environment(env), Loc::Unknown))
}

fn eval_primitive_eval(env: SmartEnv, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let expr = args.remove(0);
    trampoline::run(eval::eval_node, env, expr)
}
