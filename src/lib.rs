mod nodes;
mod parser;
mod scanner;
mod tokens;

use nodes::Node;

pub fn interpret(input: &str) -> String {
    let parse_result = parser::parse(input);

    match parse_result {
        Ok(nodes) => {
            let result = eval(nodes);
            result.display()
        }
        Err(mut syntax_errors) => {
            syntax_errors.remove(0) //TODO
        }
    }
}

fn eval(nodes: Vec<Node>) -> Node {
    let mut result = Node::Error; // TODO: should this be nil?

    for node in nodes {
        match node {
            Node::List(children) => {
                result = eval_list(children);
            }
            n @ Node::Number(_) => {
                result = n;
            }
            _ => {
                result = Node::Error;
            }
        };
    }

    result
}

fn eval_list(mut children: Vec<Node>) -> Node {
    if children[0] == Node::Symbol("quote".to_string()) {
        children.remove(1)
    } else {
        Node::Error
    }
}
