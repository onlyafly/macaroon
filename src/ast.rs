use loc::Loc;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Error(String),
    Number(i32),
    Symbol(String),
    Proc { params: Vec<Node>, body: Vec<Node> },
    List { children: Vec<Node> }, // NOTE: you might need to box the interior object
}

/*
pub struct NodeWithLoc {
    node: Node,
    loc: Loc,
}
*/

// TODO: add loc: Option<Loc>

impl Node {
    pub fn display(&self) -> String {
        #[allow(unreachable_patterns)]
        match self {
            &Node::Error(ref s) => format!("<error: {}>", s),
            &Node::Number(n) => n.to_string(),
            &Node::Symbol(ref s) => s.clone(),
            &Node::List { ref children } => {
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
