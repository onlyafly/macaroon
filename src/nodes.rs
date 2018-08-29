#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub enum Node {
    Error,
    Number(i32),
    Symbol(String),
    List(Vec<Node>),
}

impl Node {
    pub fn display(self) -> String {
        #[allow(unreachable_patterns)]
        match self {
            Node::Error => "<error>".to_string(),
            Node::Number(n) => n.to_string(),
            Node::Symbol(s) => s,
            Node::List(children) => {
                let mut v = Vec::new();
                for child in children {
                    v.push(child.display());
                }
                "(".to_string() + &v.join(" ") + ")"
            }

            n => format!("<unrecognized node: {:?}>", n),
        }
    }
}
