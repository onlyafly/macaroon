pub mod env;
mod eval;
mod primitives;
pub mod runtime_error;
mod specials;
mod trampoline;

use ast::{Node, ReaderObj, Val, WriterObj};
use back::env::{Env, SmartEnv};
use back::eval::NodeResult;
use back::runtime_error::RuntimeError;
use loc::Loc;
use std::rc::Rc;

pub fn create_root_env(writer: WriterObj, reader: ReaderObj) -> Result<SmartEnv, RuntimeError> {
    let env = Env::new(None);

    env.borrow_mut()
        .define("*writer*", Node::new(Val::Writer(writer), Loc::Unknown))?;

    env.borrow_mut()
        .define("*reader*", Node::new(Val::Reader(reader), Loc::Unknown))?;

    primitives::init_env_with_primitives(&env)?;
    Ok(env)
}

pub fn eval(env: SmartEnv, values: Vec<Node>) -> NodeResult {
    let mut output = Node::new(Val::Error("NO-INPUT".to_string()), Loc::Unknown); // TODO: should this be nil?

    for val in values {
        output = trampoline::run(eval::eval_node, Rc::clone(&env), val)?;
    }

    Ok(output)
}
