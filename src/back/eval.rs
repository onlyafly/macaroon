use ast::{Node, Value};
#[allow(unused_imports)]
use back::env::{Env, SmartEnv};
use back::primitives::eval_primitive_by_name;
use back::runtime_error::RuntimeError;
use back::specials;
use loc::Loc;

type EvalResult = Result<Node, RuntimeError>;

pub fn eval_node(env: &SmartEnv, node: Node) -> EvalResult {
    let loc = node.loc;
    match node.value {
        Value::List { children } => eval_list(env, children),
        Value::Symbol(name) => match env.borrow_mut().get(&name) {
            Some(node) => Ok(node),
            None => Err(RuntimeError::UndefinedName(name, loc)),
        },
        n @ Value::Number(_) => Ok(Node::new(n, loc)),
        n => Err(RuntimeError::UnableToEvalValue(n, loc)),
    }
}

fn eval_each_node(env: &SmartEnv, nodes: Vec<Node>) -> Result<Vec<Node>, RuntimeError> {
    let mut outputs = Vec::new();
    for node in nodes {
        let output = eval_node(env, node)?;
        outputs.push(output);
    }
    Ok(outputs)
}

fn eval_list(env: &SmartEnv, mut args: Vec<Node>) -> EvalResult {
    let head_node = args.remove(0);
    let head_value = head_node.value;
    let loc = head_node.loc;

    match head_value {
        Value::Symbol(ref name) => match name.as_ref() {
            "list" => {
                check_builtin_args("list", &loc, &args, 0, -1)?;
                return specials::eval_special_list(env, loc, args);
            }
            "quote" => {
                check_builtin_args("quote", &loc, &args, 1, -1)?;
                return specials::eval_special_quote(args);
            }
            "def" => {
                check_builtin_args("def", &loc, &args, 2, 2)?;
                return specials::eval_special_def(env, args);
            }
            "fn" => {
                check_builtin_args("fn", &loc, &args, 2, 2)?;
                return specials::eval_special_fn(env, args);
            }
            "update!" => {
                check_builtin_args("update!", &loc, &args, 2, 2)?;
                return specials::eval_special_update(env, args);
            }
            "update-element!" => {
                check_builtin_args("update-element!", &loc, &args, 3, 3)?;
                return specials::eval_special_update_element(env, args);
            }
            "if" => {
                check_builtin_args("if", &loc, &args, 3, 3)?;
                return specials::eval_special_if(env, args);
            }
            "let" => {
                check_builtin_args("let", &loc, &args, 2, -1)?;
                return specials::eval_special_let(env, args);
            }
            _ => {}
        },
        _ => {}
    }

    let evaled_head = eval_node(env, Node::new(head_value, loc.clone()))?;

    match evaled_head.value {
        Value::Function { .. } => eval_invoke_proc(env, evaled_head, args),
        Value::Primitive { primitive_name, .. } => eval_invoke_primitive(primitive_name, env, args),
        _ => Err(RuntimeError::UnableToEvalListStartingWith(
            evaled_head.display(),
            loc,
        )),
    }
}

fn eval_invoke_primitive(
    primitive_name: String,
    dynamic_env: &SmartEnv,
    unevaled_args: Vec<Node>,
) -> EvalResult {
    let evaled_args = eval_each_node(dynamic_env, unevaled_args)?;
    eval_primitive_by_name(primitive_name, dynamic_env, evaled_args)
}

fn eval_invoke_proc(dynamic_env: &SmartEnv, proc: Node, unevaled_args: Vec<Node>) -> EvalResult {
    let loc = proc.loc;
    match proc.value {
        Value::Function {
            params,
            body,
            lexical_env: parent_lexical_env,
        } => {
            // Validate params
            if unevaled_args.len() != params.len() {
                return Err(RuntimeError::ProcArgsDoNotMatchParams(String::new(), loc));
            }

            // Create the lexical environment based on the procedure's lexical parent
            let lexical_env = Env::new(Some(parent_lexical_env));

            // Prepare the arguments for evaluation
            let mut evaled_args = eval_each_node(dynamic_env, unevaled_args)?;

            // Map arguments to parameters
            for param in params {
                let evaled_arg = match evaled_args.pop() {
                    None => return Err(RuntimeError::Unknown("not enough args".to_string(), loc)),
                    Some(n) => n,
                };

                match param.value {
                    Value::Symbol(name) => {
                        lexical_env.borrow_mut().define(&name, evaled_arg)?;
                    }
                    _ => return Err(RuntimeError::Unknown("param not a symbol".to_string(), loc)),
                }
            }

            // Evaluate the application of the procedure
            eval_node(&lexical_env, *body)
        }
        _ => panic!("Cannot invoke a non-procedure"),
    }

    /* TODO
    defer func() {
		if e := recover(); e != nil {
			switch errorValue := e.(type) {
			case *EvalError:
				fmt.Printf("TRACE: (%v: %v): call to %v\n", head.Loc().Filename, head.Loc().Line, f.Name)
				panic(errorValue)
			default:
				panic(errorValue)
			}
		}
	}()
    */
}

#[allow(dead_code, unused_variables)]
fn check_builtin_args(
    name: &str,
    loc: &Loc,
    args: &Vec<Node>,
    min_params: isize,
    max_params: isize,
) -> Result<(), RuntimeError> {
    if max_params == -1 {
        if (args.len() as isize) < min_params {
            return Err(RuntimeError::NotEnoughArgs(
                name.to_string(),
                min_params,
                args.len(),
                loc.clone(),
            ));
        }
    } else if (min_params == max_params) && (min_params != args.len() as isize) {
        return Err(RuntimeError::WrongNumberOfArgs(
            name.to_string(),
            min_params,
            args.len(),
            loc.clone(),
        ));
    } else if ((args.len() as isize) < min_params) || ((args.len() as isize) > max_params) {
        return Err(RuntimeError::ArgCountOutOfRange(
            name.to_string(),
            min_params,
            max_params,
            args.len(),
            loc.clone(),
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_builtin_args() {
        // Arrange
        //let args = vec![Node::new(Value::Number(42), Loc::Unknown)];
        let args = Vec::<Node>::new();

        // Act
        let r = check_builtin_args("list", &Loc::Unknown, &args, 1, -1);

        // Assert
        assert_eq!(
            r,
            Err(RuntimeError::NotEnoughArgs(
                "list".to_string(),
                1,
                0,
                Loc::Unknown
            ))
        );
    }
}
