use ast::{Node, Value};
use back::env::Env;
use back::runtime_error::RuntimeError;
use back::specials;
use loc::Loc;

pub fn eval_node(env: &mut Env, node: Node) -> Result<Node, RuntimeError> {
    let loc = node.loc;
    match node.value {
        Value::List { children } => eval_list(env, children),
        Value::Symbol(name) => match env.get(&name) {
            Some(&ref node) => Ok(Node::new(node.value.clone(), loc)),
            None => Err(RuntimeError::UndefinedName(name, loc)),
        },
        n @ Value::Number(_) => Ok(Node::new(n, loc)),
        n => Err(RuntimeError::UnableToEvalValue(n, loc)),
    }
}

fn eval_list(env: &mut Env, mut args: Vec<Node>) -> Result<Node, RuntimeError> {
    let node = args.remove(0);
    let value = node.value;
    let loc = node.loc;

    match value {
        Value::Symbol(ref name) => match name.as_ref() {
            "list" => {
                check_builtin_args("list", &loc, &args, 0, -1)?;
                specials::eval_special_list(env, loc, args)
            }
            "quote" => {
                check_builtin_args("quote", &loc, &args, 1, -1)?;
                specials::eval_special_quote(args)
            }
            "def" => specials::eval_special_def(env, args),
            "fn" => specials::eval_special_fn(env, args),
            "update!" => specials::eval_special_update(env, args),
            "update-element!" => specials::eval_special_update_element(env, args),
            _ => Err(RuntimeError::UnableToEvalListStartingWith(
                name.clone(),
                loc,
            )),
        },
        n => {
            let evaluated_head = eval_node(env, Node::new(n, loc.clone()))?;

            match evaluated_head.value {
                Value::Proc { mut body, .. } => {
                    Ok(body.remove(0)) // TODO: we currently just return the first item in the body
                }
                _ => Err(RuntimeError::UnableToEvalListStartingWith(
                    evaluated_head.display(),
                    loc,
                )),
            }
        }
    }
}

/*
func checkSpecialArgs(name string, head ast.Node, args []ast.Node, paramCountMin int, paramCountMax int) {
	checkBuiltinArgs("Special form", name, head, args, paramCountMin, paramCountMax)
}

func checkPrimitiveArgs(name string, head ast.Node, args []ast.Node, paramCountMin int, paramCountMax int) {
	checkBuiltinArgs("Primitive", name, head, args, paramCountMin, paramCountMax)
}
*/

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
    }
    /*
	switch {
	case paramCountMax == -1:
		
	case paramCountMin == paramCountMax:
		if !(paramCountMin == len(args)) {
			panicEvalError(head, fmt.Sprintf(
				"%v '%v' expects %v argument(s), but was given %v",
				builtinType,
				name,
				paramCountMin,
				len(args)))
		}
	default:
		if !(paramCountMin <= len(args) && len(args) <= paramCountMax) {
			panicEvalError(head, fmt.Sprintf(
				"%v '%v' expects between %v and %v arguments, but was given %v",
				builtinType,
				name,
				paramCountMin,
				paramCountMax,
				len(args)))
		}
	}
    */
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
