use loc::Loc;
use std::ops::Deref;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Error(String),
    Number(i32),
    Symbol(String),
    Proc { params: Vec<Node>, body: Vec<Node> },
    List { children: Vec<Node> },
}

#[derive(PartialEq, Debug, Clone)]
pub struct Node {
    pub value: Value,
    pub loc: Loc,
}

impl Node {
    pub fn new(value: Value, loc: Loc) -> Self {
        Node { value, loc }
    }
}

impl Deref for Node {
    type Target = Value;

    fn deref(&self) -> &Value {
        &self.value
    }
}

// TODO: add loc: Option<Loc>

impl Value {
    pub fn display(&self) -> String {
        #[allow(unreachable_patterns)]
        match self {
            &Value::Error(ref s) => format!("<error: {}>", s),
            &Value::Number(n) => n.to_string(),
            &Value::Symbol(ref s) => s.clone(),
            &Value::List { ref children } => {
                let mut v = Vec::new();
                for child in children {
                    v.push(child.display());
                }
                "(".to_string() + &v.join(" ") + ")"
            }
            n => format!("<unrecognized value: {:?}>", n),
        }
    }
}
