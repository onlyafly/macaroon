#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Error(String),
    Number(i32),
    Symbol(String),
    Procedure { params: Vec<Node>, body: Vec<Node> },
    List(Vec<Node>),
}

impl Node {
    pub fn display(&self) -> String {
        #[allow(unreachable_patterns)]
        match self {
            &Node::Error(ref s) => format!("<error: {}>", s),
            &Node::Number(n) => n.to_string(),
            &Node::Symbol(ref s) => s.clone(),
            &Node::List(ref children) => {
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
