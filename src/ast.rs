use loc::Loc;
use std::ops::Deref;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Error(String),
    Number(i32),
    Symbol(String),
    Proc {
        params: Vec<WrappedNode>,
        body: Vec<WrappedNode>,
    },
    List {
        children: Vec<WrappedNode>,
    },
}

#[derive(PartialEq, Debug, Clone)]
pub struct WrappedNode {
    pub node: Node,
    pub loc: Loc,
}

impl WrappedNode {
    pub fn new(node: Node, loc: Loc) -> Self {
        WrappedNode { node, loc }
    }
}

impl Deref for WrappedNode {
    type Target = Node;

    fn deref(&self) -> &Node {
        &self.node
    }
}

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
