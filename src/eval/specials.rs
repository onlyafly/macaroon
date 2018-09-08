use ast::*;
use eval;
use eval::env::Env;

pub fn eval_special_list(env: &mut Env, args: Vec<Node>) -> Result<Node, String> {
    let mut evaled_args = Vec::new();

    for child in args {
        let evaled_child = eval::eval_node(env, child)?;
        evaled_args.push(evaled_child);
    }

    Ok(Node::List(Box::new(ListObj {
        children: evaled_args,
    })))
}

pub fn eval_special_quote(mut args: Vec<Node>) -> Result<Node, String> {
    Ok(args.remove(0))
}

pub fn eval_special_def(env: &mut Env, mut args: Vec<Node>) -> Result<Node, String> {
    let name_node = args.remove(0);

    if let Node::Symbol(name) = name_node {
        if env.exists(&name) {
            return Err(format!("Cannot redefine a name: {}", name));
        }

        let value_node = super::eval_node(env, args.remove(0))?;

        env.insert(name, value_node);
        Ok(Node::Number(0)) // TODO: should be nil
    } else {
        Err(format!("Expected symbol, got {}", name_node.display()))
    }
}

pub fn eval_special_update(env: &mut Env, mut args: Vec<Node>) -> Result<Node, String> {
    let name_node = args.remove(0);

    if let Node::Symbol(name) = name_node {
        if !env.exists(&name) {
            return Err(format!("Cannot update an undefined name: {}", name));
        }
        let value = args.remove(0);
        env.insert(name, value);
        Ok(Node::Number(0)) // TODO: should be nil
    } else {
        Err(format!("Expected symbol, got {}", name_node.display()))
    }
}

pub fn eval_special_update_element(env: &mut Env, mut args: Vec<Node>) -> Result<Node, String> {
    let name_node = args.remove(0);

    if let Node::Symbol(name) = name_node {
        if !env.exists(&name) {
            return Err(format!("Cannot update an undefined name: {}", name));
        }

        let _index_node = args.remove(0);
        let value_node = super::eval_node(env, args.remove(0))?;

        if let Some(entry_node) = env.remove(&name) {
            match entry_node {
                Node::List(mut list_obj) => {
                    //TODO: get num from index_node instead of using zero
                    list_obj.children[0] = value_node;
                    env.insert(name, Node::List(list_obj));
                }
                _ => {
                    return Err(format!(
                        "Tried to update an element in a non-list: {}",
                        entry_node.display()
                    ));
                }
            }
        }

        Ok(Node::Number(0)) // TODO: should be nil
    } else {
        Err(format!("Expected symbol, got {}", name_node.display()))
    }
}

pub fn eval_special_fn(_env: &mut Env, mut args: Vec<Node>) -> Result<Node, String> {
    let param_list = args.remove(0);
    let body = args;

    match param_list {
        Node::List(list_node) => Ok(Node::Proc(Box::new(ProcObj {
            params: list_node.children,
            body: body,
        }))),
        _ => Err(format!(
            "Expected list of paramters, got {}",
            param_list.display()
        )),
    }
}
