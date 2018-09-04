use eval;
use eval::env::Env;
use nodes::Node;

pub fn eval_special_list(env: &mut Env, children: Vec<Node>) -> Result<Node, String> {
    let mut evaled_children = Vec::new();

    for child in children {
        let evaled_child = eval::eval_node(env, child)?;
        evaled_children.push(evaled_child);
    }
    Ok(Node::List(evaled_children))
}

pub fn eval_special_quote(mut children: Vec<Node>) -> Result<Node, String> {
    Ok(children.remove(0))
}

pub fn eval_special_def(env: &mut Env, mut children: Vec<Node>) -> Result<Node, String> {
    let name_node = children.remove(0);

    if let Node::Symbol(name) = name_node {
        if env.exists(&name) {
            return Err(format!("Cannot redefine a name: {}", name));
        }
        let value = children.remove(0);
        env.insert(name, value);
        Ok(Node::Number(0)) // TODO: should be nil
    } else {
        Err(format!("Expected symbol, got {}", name_node.display()))
    }
}

pub fn eval_special_update(env: &mut Env, mut children: Vec<Node>) -> Result<Node, String> {
    let name_node = children.remove(0);

    if let Node::Symbol(name) = name_node {
        if !env.exists(&name) {
            return Err(format!("Cannot update an undefined name: {}", name));
        }
        let value = children.remove(0);
        env.insert(name, value);
        Ok(Node::Number(0)) // TODO: should be nil
    } else {
        Err(format!("Expected symbol, got {}", name_node.display()))
    }
}

pub fn eval_special_fn(_env: &mut Env, mut children: Vec<Node>) -> Result<Node, String> {
    let param_list = children.remove(0);
    let body = children;

    match param_list {
        Node::List(params) => Ok(Node::Procedure {
            params: params,
            body: body,
        }),
        _ => Err(format!(
            "Expected list of paramters, got {}",
            param_list.display()
        )),
    }
}
