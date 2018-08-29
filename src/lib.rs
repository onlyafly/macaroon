mod nodes;
mod parser;
mod scanner;
mod tokens;

use nodes::Node;

pub fn interpret(input: &str) -> String {
    let parse_result = parser::parse(input);

    match parse_result {
        Ok(nodes) => match eval(nodes) {
            Ok(output_node) => output_node.display(),
            Err(message) => message,
        },
        Err(mut syntax_errors) => {
            syntax_errors.remove(0) //TODO
        }
    }
}

fn eval(nodes: Vec<Node>) -> Result<Node, String> {
    let mut output_node = Node::Error; // TODO: should this be nil?

    for node in nodes {
        match node {
            Node::List(children) => {
                output_node = eval_list(children);
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

fn eval_list(mut children: Vec<Node>) -> Node {
    if children[0] == Node::Symbol("quote".to_string()) {
        children.remove(1)
    } else {
        Node::Error
    }
}
