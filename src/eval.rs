use env::Env;
use nodes::Node;

pub fn eval(env: &mut Env, nodes: Vec<Node>) -> Result<Node, String> {
    let mut output_node = Node::Error; // TODO: should this be nil?

    for node in nodes {
        match node {
            Node::List(children) => {
                output_node = eval_list(env, children)?;
            }
            n @ Node::Number(_) => {
                output_node = n;
            }
            n => {
                return Err(format!("Unable to eval node: {}", n.display()));
            }
        };
    }

    Ok(output_node)
}

fn eval_list(env: &mut Env, mut children: Vec<Node>) -> Result<Node, String> {
    match children.remove(0) {
        Node::Symbol(ref name) => match name.as_ref() {
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
