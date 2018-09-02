use env::Env;
use nodes::Node;

pub fn eval(env: &mut Env, nodes: Vec<Node>) -> Result<Node, String> {
    let mut output_node = Node::Error("NO-INPUT".to_string()); // TODO: should this be nil?

    for node in nodes {
        output_node = eval_node(env, node)?;
    }

    Ok(output_node)
}

fn eval_node(env: &mut Env, node: Node) -> Result<Node, String> {
    match node {
        Node::List(children) => eval_list(env, children),
        Node::Symbol(name) => match env.get(&name) {
            Some(&ref node) => Ok(node.clone()),
            None => Err(format!("Undefined symbol: {}", name)),
        },
        n @ Node::Number(_) => Ok(n),
        n => Err(format!("Unable to eval node: {}", n.display())),
    }
}

fn eval_list(env: &mut Env, mut children: Vec<Node>) -> Result<Node, String> {
    match children.remove(0) {
        Node::Symbol(ref name) => match name.as_ref() {
            "list" => eval_special_list(env, children),
            "quote" => eval_special_quote(children),
            "def" => eval_special_def(env, children),
            _ => Err(format!(
                "Don't know what to do with list starting with: {}",
                name
            )),
        },
        n => Err(format!(
            "Don't know what to do with list starting with: {}",
            n.display()
        )),
    }
}

fn eval_special_list(env: &mut Env, children: Vec<Node>) -> Result<Node, String> {
    let mut evaled_children = Vec::new();

    for child in children {
        let evaled_child = eval_node(env, child)?;
        evaled_children.push(evaled_child);
    }
    Ok(Node::List(evaled_children))
}

fn eval_special_quote(mut children: Vec<Node>) -> Result<Node, String> {
    Ok(children.remove(0))
}

fn eval_special_def(env: &mut Env, mut children: Vec<Node>) -> Result<Node, String> {
    let name_node = children.remove(0);

    if let Node::Symbol(name) = name_node {
        let value = children.remove(0);
        env.insert(name, value);
        Ok(Node::Number(0)) // TODO: should be nil
    } else {
        Err(format!("Expected symbol, got {}", name_node.display()))
    }
}
