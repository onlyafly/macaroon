use tokens;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Error(String),
    Number(i32),
    Symbol(String),
    Proc(Box<ProcObj>),
    List(Box<ListObj>),
}

pub trait Obj {
    fn loc(&self) -> tokens::Loc;
}

#[derive(PartialEq, Clone, Debug)]
pub struct ProcObj {
    pub params: Vec<Node>,
    pub body: Vec<Node>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ListObj {
    pub children: Vec<Node>,
}

impl Node {
    pub fn display(&self) -> String {
        #[allow(unreachable_patterns)]
        match self {
            &Node::Error(ref s) => format!("<error: {}>", s),
            &Node::Number(n) => n.to_string(),
            &Node::Symbol(ref s) => s.clone(),
            &Node::List(ref list_node) => {
                let mut v = Vec::new();
                for child in &list_node.children {
                    v.push(child.display());
                }
                "(".to_string() + &v.join(" ") + ")"
            }
            n => format!("<unrecognized node: {:?}>", n),
        }
    }
}