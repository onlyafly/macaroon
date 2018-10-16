use ast::{Node, RoutineObj, RoutineType, Val};
use back::env::{Env, SmartEnv};
use back::eval;
use back::runtime_error::RuntimeError;
use back::trampoline;
use back::trampoline::{ContinuationResult, Flag};
use loc::Loc;
use std::rc::Rc;

pub fn eval_special_list(env: SmartEnv, loc: Loc, args: Vec<Node>) -> ContinuationResult {
    let mut evaled_args = Vec::new();

    for child in args {
        let evaled_child = trampoline::run(eval::eval_node, Rc::clone(&env), child)?;
        evaled_args.push(evaled_child);
    }

    Ok(trampoline::finish(Node::new(Val::List(evaled_args), loc)))
}

pub fn eval_special_quote(mut args: Vec<Node>) -> ContinuationResult {
    Ok(trampoline::finish(args.remove(0)))
}

pub fn eval_special_def(env: SmartEnv, mut args: Vec<Node>) -> ContinuationResult {
    let name_node = args.remove(0);

    if let Val::Symbol(name) = name_node.val {
        let mut value_node = trampoline::run(eval::eval_node, Rc::clone(&env), args.remove(0))?;

        // If it is a function, give the function a name
        match value_node.val {
            Val::Routine(ref mut fobj) => fobj.name = Some(name.clone()),
            _ => (),
        }

        env.borrow_mut().define(&name, value_node)?;
        Ok(trampoline::finish(Node::new(Val::Nil, name_node.loc))) // TODO: should be nil
    } else {
        Err(RuntimeError::UnexpectedValue(
            "symbol".to_string(),
            name_node.val,
            name_node.loc,
        ))
    }
}

pub fn eval_special_let(env: SmartEnv, mut args: Vec<Node>) -> ContinuationResult {
    let bindings_node = args.remove(0);

    match bindings_node.val {
        Val::List(mut bindings_vec) => {
            let bindings_env = Env::new(Some(Rc::clone(&env)));

            while bindings_vec.len() > 1 {
                let name_node = bindings_vec.remove(0);

                // By evaluating the value in the bindings environment, this allows recursive definitions
                let value_node = trampoline::run(
                    eval::eval_node,
                    Rc::clone(&bindings_env),
                    bindings_vec.remove(0),
                )?;

                let name = match name_node.val {
                    Val::Symbol(name) => name,
                    v => {
                        return Err(RuntimeError::UnexpectedValue(
                            "symbol".to_string(),
                            v,
                            name_node.loc,
                        ))
                    }
                };

                bindings_env.borrow_mut().define(&name, value_node)?;
            }

            Ok(trampoline::bounce(
                eval::eval_node,
                bindings_env,
                args.remove(0),
            ))
        }
        _ => Err(RuntimeError::UnexpectedValue(
            "let".to_string(),
            bindings_node.val,
            bindings_node.loc,
        )),
    }
}

pub fn eval_special_update(env: SmartEnv, mut args: Vec<Node>) -> ContinuationResult {
    let name_node = args.remove(0);
    let val = name_node.val;
    let loc = name_node.loc;

    if let Val::Symbol(name) = val {
        let val = trampoline::run(eval::eval_node, Rc::clone(&env), args.remove(0))?;
        env.borrow_mut().update(&name, val)?;
        Ok(trampoline::finish(Node::new(Val::Nil, loc)))
    } else {
        Err(RuntimeError::UnexpectedValue(
            "symbol".to_string(),
            val,
            loc,
        ))
    }
}

pub fn eval_special_update_element(env: SmartEnv, mut args: Vec<Node>) -> ContinuationResult {
    let name_node = args.remove(0);
    let loc = name_node.loc;

    if let Val::Symbol(name) = name_node.val {
        let mut index_node = trampoline::run(eval::eval_node, Rc::clone(&env), args.remove(0))?;
        let index = index_node.as_host_number()? as usize;

        let newval_node = trampoline::run(eval::eval_node, Rc::clone(&env), args.remove(0))?;

        let mut mutable_env = env.borrow_mut();

        if let Some(entry) = mutable_env.remove(&name) {
            match entry.val {
                Val::List(mut children) => {
                    //TODO: get num from index_value instead of using zero

                    if index >= children.len() {
                        return Err(RuntimeError::IndexOutOfBounds {
                            index: index,
                            len: children.len(),
                            loc: loc,
                        });
                    }

                    children[index] = newval_node;
                    mutable_env.update(&name, Node::new(Val::List(children), loc.clone()))?;
                }
                _ => {
                    return Err(RuntimeError::CannotUpdateElementInValue(entry.val, loc));
                }
            }
        }

        Ok(trampoline::finish(Node::new(Val::Nil, loc)))
    } else {
        Err(RuntimeError::UnexpectedValue(
            "symbol".to_string(),
            name_node.val,
            loc,
        ))
    }
}

pub fn eval_special_if(env: SmartEnv, mut args: Vec<Node>) -> ContinuationResult {
    let predicate = trampoline::run(eval::eval_node, Rc::clone(&env), args.remove(0))?;
    let true_branch = args.remove(0);
    let false_branch = args.remove(0);

    let branch = match predicate.as_host_boolean()? {
        true => true_branch,
        false => false_branch,
    };

    Ok(trampoline::bounce(eval::eval_node, env, branch))
}

pub fn eval_special_cond(env: SmartEnv, mut args: Vec<Node>) -> ContinuationResult {
    loop {
        match args.len() {
            0 => return Ok(trampoline::finish(Node::new(Val::Nil, Loc::Unknown))),
            1 => {
                let unmatched_node = args.remove(0);
                return Err(RuntimeError::CondUnmatchedClause(
                    unmatched_node.val,
                    unmatched_node.loc,
                ));
            }
            _ => (),
        }

        let predicate = trampoline::run(eval::eval_node, Rc::clone(&env), args.remove(0))?;
        let unevaled_branch = args.remove(0);

        if predicate.as_host_boolean()? {
            return Ok(trampoline::bounce(eval::eval_node, env, unevaled_branch));
        }
    }
}

pub fn eval_special_for(env: SmartEnv, mut args: Vec<Node>) -> ContinuationResult {
    let name_node = args.remove(0);
    let loc = name_node.loc;
    let name = match name_node.val {
        Val::Symbol(name) => name,
        v => return Err(RuntimeError::UnexpectedValue("symbol".to_string(), v, loc)),
    };

    let start_node = trampoline::run(eval::eval_node, Rc::clone(&env), args.remove(0))?;
    let mut start_number = start_node.as_host_number()?;

    let end_node = trampoline::run(eval::eval_node, Rc::clone(&env), args.remove(0))?;
    let end_number = end_node.as_host_number()?;

    let body = args.remove(0);

    let mut output = Node::new(Val::Nil, loc.clone());

    while start_number <= end_number {
        let loop_env = Env::new(Some(Rc::clone(&env)));
        let index_node = Node::new(Val::Number(start_number), loc.clone());
        loop_env.borrow_mut().define(&name, index_node)?;

        let cloned_body = body.clone();
        output = trampoline::run(eval::eval_node, loop_env, cloned_body)?;

        start_number += 1;
    }

    Ok(trampoline::finish(output))
}

pub fn eval_special_begin(env: SmartEnv, unevaled_args: Vec<Node>) -> ContinuationResult {
    let output = eval::eval_each_node_for_single_output(env, unevaled_args)?;
    Ok(trampoline::finish(output))
}

pub fn eval_special_routine(
    lexical_env: SmartEnv,
    mut args: Vec<Node>,
    routine_type: RoutineType,
) -> ContinuationResult {
    let param_list = args.remove(0);
    let body = args.remove(0); // The body is only one node

    match param_list.val {
        Val::List(children) => Ok(trampoline::finish(Node::new(
            Val::Routine(RoutineObj {
                name: None,
                params: children,
                body: Box::new(body),
                lexical_env: Rc::clone(&lexical_env),
                routine_type,
            }),
            param_list.loc,
        ))),
        _ => Err(RuntimeError::UnexpectedValue(
            "list of parameters".to_string(),
            param_list.val,
            param_list.loc,
        )),
    }
}

pub fn eval_special_macroexpand1(env: SmartEnv, mut args: Vec<Node>) -> ContinuationResult {
    let unexpanded_node = trampoline::run(eval::eval_node, Rc::clone(&env), args.remove(0))?;

    match unexpanded_node.val {
        Val::List(..) => {
            let output = trampoline::run_with_flag(
                eval::eval_list,
                env,
                unexpanded_node,
                Flag::DelayMacroEvaluation,
            )?;
            Ok(trampoline::finish(output))
        }
        _ => Err(RuntimeError::UnexpectedValue(
            "list".to_string(),
            unexpanded_node.val,
            unexpanded_node.loc,
        )),
    }
}
