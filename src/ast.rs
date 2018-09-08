#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum Node {
    Error(String),
    Number(i32),
    Symbol(String),
    Proc(ProcObj), // NOTE: you might need to box the interior object
    List(ListObj), // NOTE: you might need to box the interior object
}

impl Node {
    pub fn display(&self) -> String {
        #[allow(unreachable_patterns)]
        match self {
            &Node::Error(ref s) => format!("<error: {}>", s),
            &Node::Number(n) => n.to_string(),
            &Node::Symbol(ref s) => s.clone(),
            &Node::List(ref list_obj) => {
                let mut v = Vec::new();
                for child in &list_obj.children {
                    v.push(child.display());
                }
                "(".to_string() + &v.join(" ") + ")"
            }
            n => format!("<unrecognized node: {:?}>", n),
        }
    }
}

/* TODO: Is this needed?
use tokens;
pub trait Obj {
    fn loc(&self) -> tokens::Loc;
}
*/

#[derive(PartialEq, Clone, Debug)]
pub struct ProcObj {
    pub params: Vec<Node>,
    pub body: Vec<Node>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct ListObj {
    pub children: Vec<Node>,
}
