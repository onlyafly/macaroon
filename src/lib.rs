mod ast;
mod parser;
mod scanner;
mod tokens;

use ast::Node;

pub fn interpret(input: &str) -> Node {
    let mut p = parser::Parser::new(input);
    let nodes = p.parse();
    let result = eval(nodes);
    result
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
