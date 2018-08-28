#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Node {
    Error,
    Number(i32),
    Symbol(String),
    List(Vec<Node>),
}

impl Node {
    pub fn display(self) -> String {
        match self {
            Node::Error => "<error>".to_string(),
            Node::Number(n) => n.to_string(),
            _ => "<unknown>".to_string(),
        }
    }
}
