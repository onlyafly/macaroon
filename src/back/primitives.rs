/* Primitives are build-in functions */

use ast::{CellObj, Node, PrimitiveFnPointer, PrimitiveObj, ReaderObj, Val, WriterObj};
use back::env::{Env, SmartEnv};
use back::eval;
use back::eval::NodeResult;
use back::runtime_error::RuntimeError;
use back::trampoline;
use front;
use loc::Loc;
use std::cell::RefMut;
use std::fs::File;
use std::io::prelude::*;

pub fn init_env_with_primitives(env: &SmartEnv) -> Result<(), RuntimeError> {
    let mut menv = env.borrow_mut();

    menv.define("true", Node::new(Val::Boolean(true), Loc::Unknown))?;
    menv.define("false", Node::new(Val::Boolean(false), Loc::Unknown))?;
    menv.define("nil", Node::new(Val::Nil, Loc::Unknown))?;

    def_prim(&mut menv, "+", prim_add, 2, 2)?; // TODO: should be 0, -1
    def_prim(&mut menv, "-", prim_subtract, 2, 2)?; // TODO: should be 1, -1 ???
    def_prim(&mut menv, "=", prim_equal, 2, 2)?; // TODO: should be 2, -1
    def_prim(&mut menv, "<", prim_less_than, 2, 2)?;
    def_prim(&mut menv, ">", prim_greater_than, 2, 2)?;

    def_prim(&mut menv, "panic", prim_panic, 0, -1)?;
    def_prim(&mut menv, "read-line", prim_read_line, 0, 0)?;
    def_prim(&mut menv, "print", prim_print, 0, -1)?;
    def_prim(&mut menv, "println", prim_println, 0, -1)?;
    def_prim(&mut menv, "not", prim_not, 1, 1)?;
    def_prim(&mut menv, "apply", prim_apply, 2, 2)?;
    def_prim(&mut menv, "typeof", prim_typeof, 1, 1)?;
    def_prim(&mut menv, "load", prim_load, 1, 1)?;

    def_prim(&mut menv, "str", prim_str, 0, -1)?;
    def_prim(&mut menv, "concat", prim_concat, 0, -1)?;
    def_prim(&mut menv, "cons", prim_cons, 2, 2)?;
    def_prim(&mut menv, "first", prim_first, 1, 1)?;
    def_prim(&mut menv, "rest", prim_rest, 1, 1)?;
    def_prim(&mut menv, "len", prim_len, 1, 1)?;
    def_prim(&mut menv, "trim-string", prim_trim_string, 1, 1)?;

    def_prim(
        &mut menv,
        "current-environment",
        prim_current_environment,
        0,
        0,
    )?;
    def_prim(&mut menv, "eval", prim_eval, 1, 2)?;
    def_prim(&mut menv, "read-string", prim_read_string, 1, 1)?;
    def_prim(&mut menv, "readable-string", prim_readable_string, 1, 1)?;

    def_prim(&mut menv, "cell", prim_cell, 1, 1)?;
    def_prim(&mut menv, "set-cell!", prim_set_cell, 2, 2)?;
    def_prim(&mut menv, "get-cell", prim_get_cell, 1, 1)?;

    def_prim(&mut menv, "_host_inspect_", prim_host_inspect, 1, 1)?;
    def_prim(&mut menv, "_host_backtrace_", prim_host_backtrace, 0, 0)?;

    Ok(())
}

fn def_prim(
    mut_env: &mut RefMut<Env>,
    name: &'static str,
    f: PrimitiveFnPointer,
    min_arity: isize,
    max_arity: isize,
) -> Result<(), RuntimeError> {
    mut_env.define(
        name,
        Node::new(
            Val::Primitive(PrimitiveObj {
                name: name.to_string(),
                f: f,
                min_arity,
                max_arity,
            }),
            Loc::Unknown,
        ),
    )
}

fn prim_not(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let one = args.remove(0);

    let one_bool = one.as_host_boolean()?;

    let output = !one_bool;

    let result = Node::new(Val::Boolean(output), one.loc);
    Ok(result)
}

fn prim_add(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let one = args.remove(0);
    let two = args.remove(0);

    let one_i32 = one.as_host_number()?;
    let two_i32 = two.as_host_number()?;

    let output = one_i32 + two_i32;

    let result = Node::new(Val::Number(output), one.loc);
    Ok(result)
}

fn prim_subtract(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let one = args.remove(0);
    let two = args.remove(0);

    let one_i32 = one.as_host_number()?;
    let two_i32 = two.as_host_number()?;

    let output = one_i32 - two_i32;

    let result = Node::new(Val::Number(output), one.loc);
    Ok(result)
}

fn prim_equal(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let a = args.remove(0);
    let b = args.remove(0);

    let output = a.val == b.val;

    Ok(Node::new(Val::Boolean(output), a.loc))
}

fn prim_less_than(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let a = args.remove(0);
    let b = args.remove(0);

    let output = a.val < b.val;

    Ok(Node::new(Val::Boolean(output), a.loc))
}

fn prim_greater_than(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let a = args.remove(0);
    let b = args.remove(0);

    let output = a.val > b.val;

    Ok(Node::new(Val::Boolean(output), a.loc))
}

fn prim_panic(_env: SmartEnv, args: Vec<Node>) -> NodeResult {
    let mut v = Vec::new();
    let mut loc = Loc::Unknown;
    for arg in args {
        loc = arg.loc.clone();
        v.push(arg.as_print_friendly_string());
    }
    let output = format!("{}\n", &v.join(" "));

    Err(RuntimeError::ApplicationPanic(output, loc))
}

fn prim_read_line(env: SmartEnv, _args: Vec<Node>) -> NodeResult {
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

fn prim_print(env: SmartEnv, args: Vec<Node>) -> NodeResult {
    prim_print_or_println(env, args, false)
}

fn prim_println(env: SmartEnv, args: Vec<Node>) -> NodeResult {
    prim_print_or_println(env, args, true)
}

fn prim_print_or_println(env: SmartEnv, args: Vec<Node>, add_newline: bool) -> NodeResult {
    let mut v = Vec::new();
    let mut loc = Loc::Unknown;
    for arg in args {
        loc = arg.loc.clone();
        v.push(arg.as_print_friendly_string());
    }

    let output = if add_newline {
        format!("{}\n", &v.join(" "))
    } else {
        format!("{}", &v.join(" "))
    };

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

fn prim_apply(env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let f = args.remove(0);
    let f_args_node = args.remove(0);
    let f_args = match f_args_node.val {
        Val::List(children) => children,
        v => {
            return Err(RuntimeError::UnexpectedValue(
                "list".to_string(),
                v,
                f_args_node.loc,
            ))
        }
    };

    let output = trampoline::run_with_nodes(eval::eval_invoke_procedure, env, f, f_args)?;

    Ok(output)
}

fn prim_typeof(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let arg = args.remove(0);

    let output = arg.type_name()?;

    Ok(Node::new(Val::Symbol(output), arg.loc))
}

fn prim_load(env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
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
    if output != "nil".to_string() {
        println!("{}", output);
    }

    Ok(Node::new(Val::Nil, filename_node.loc))
}

fn prim_str(_env: SmartEnv, args: Vec<Node>) -> NodeResult {
    let mut v = Vec::new();
    let mut loc = Loc::Unknown;
    for arg in args {
        loc = arg.loc.clone();
        v.push(arg.as_print_friendly_string());
    }
    let output = format!("{}", &v.join(""));

    Ok(Node::new(Val::StringVal(output), loc))
}

fn prim_concat(_env: SmartEnv, args: Vec<Node>) -> NodeResult {
    let mut output = Node::new(Val::Nil, Loc::Unknown);
    for mut arg in args {
        output = output.coll_append(arg)?;
    }

    Ok(output)
}

fn prim_cons(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let elem = args.remove(0);
    let coll = args.remove(0);

    let output = coll.coll_cons(elem)?;

    Ok(output)
}

fn prim_first(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    Ok(args.remove(0).coll_first()?)
}

fn prim_rest(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    Ok(args.remove(0).coll_rest()?)
}

fn prim_len(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let n = args.remove(0);
    let loc = n.loc.clone();

    let out = n.coll_len()?;

    Ok(Node::new(Val::Number(out as i32), loc))
}

fn prim_trim_string(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let node = args.remove(0);
    match node.val {
        Val::StringVal(s) => Ok(Node::new(Val::StringVal(s.trim().to_string()), node.loc)),
        v => Err(RuntimeError::UnexpectedArgumentType {
            procedure_name: "trim-string".to_string(),
            expected_type_name: "string".to_string(),
            actual_val: v,
            loc: node.loc,
        }),
    }
}

fn prim_current_environment(env: SmartEnv, _args: Vec<Node>) -> NodeResult {
    Ok(Node::new(Val::Environment(env), Loc::Unknown))
}

fn prim_eval(env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let expr = args.remove(0);

    let evaluation_env = if args.len() > 0 {
        let n = args.remove(0);
        match n.val {
            Val::Environment(e) => e,
            v => {
                return Err(RuntimeError::UnexpectedValue(
                    "environment".to_string(),
                    v,
                    n.loc,
                ))
            }
        }
    } else {
        env
    };

    trampoline::run(eval::eval_node, evaluation_env, expr)
}

fn prim_read_string(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let arg = args.remove(0);

    match arg.val {
        Val::StringVal(s) => match front::parse("<eval>", &s) {
            Ok(mut nodes) => Ok(nodes.remove(0)),
            Err(mut errors) => {
                let syntax_error = errors.remove(0);
                Err(RuntimeError::SyntaxErrorDuringRead(
                    s,
                    syntax_error,
                    arg.loc,
                ))
            }
        },
        v => Err(RuntimeError::UnexpectedValue(
            "string".to_string(),
            v,
            arg.loc,
        )),
    }
}

fn prim_readable_string(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let n = args.remove(0);
    let s = format!("{}", n.val);

    Ok(Node::new(Val::StringVal(s), n.loc))
}

fn prim_cell(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let n = args.remove(0);
    let loc = n.loc.clone();
    Ok(Node::new(Val::Cell(CellObj::new(n)), loc))
}

fn prim_set_cell(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let c = args.remove(0);
    let n = args.remove(0);

    match c.val {
        Val::Cell(obj) => {
            let mut x = obj.contents.borrow_mut();
            *x = n;
            Ok(Node::new(Val::Nil, c.loc))
        }
        v => Err(RuntimeError::UnexpectedValue("cell".to_string(), v, n.loc)),
    }
}

fn prim_get_cell(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let n = args.remove(0);

    match n.val {
        Val::Cell(obj) => Ok(obj.contents.borrow().clone()),
        v => Err(RuntimeError::UnexpectedValue("cell".to_string(), v, n.loc)),
    }
}

fn prim_host_inspect(_env: SmartEnv, mut args: Vec<Node>) -> NodeResult {
    let n = args.remove(0);
    println!("{:?}", n);

    Ok(Node::new(Val::Nil, n.loc))
}

fn prim_host_backtrace(_env: SmartEnv, _args: Vec<Node>) -> NodeResult {
    extern crate backtrace;
    let bt = backtrace::Backtrace::new();

    // do_some_work();

    println!("{:?}", bt);

    Ok(Node::new(Val::Nil, Loc::Unknown))
}
