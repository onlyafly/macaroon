use back::env::SmartEnv;
use back::runtime_error::RuntimeError;
use loc::Loc;
use std::cmp::Ordering;
use std::ops::Deref;

#[derive(PartialEq, Debug, Clone)]
pub enum Val {
    Nil,
    Error(String),
    Number(i32),
    Character(String),
    StringVal(String),
    Symbol(String),
    Boolean(bool),
    Function {
        params: Vec<Node>,
        body: Box<Node>,
        lexical_env: SmartEnv,
    },
    Primitive(PrimitiveObj),
    List {
        children: Vec<Node>,
    },
}

impl Val {
    pub fn display(&self) -> String {
        match self {
            Val::Nil => "nil".to_string(),
            Val::Error(ref s) => format!("#error<{}>", s),
            Val::Number(n) => n.to_string(),
            Val::StringVal(ref s) => format!("\"{}\"", s),
            Val::Character(ref s) => match s.as_ref() {
                "\n" => r"\newline".to_string(),
                _ => format!(r"\{}", s),
            },
            Val::Symbol(ref s) => s.clone(),
            Val::List { ref children } => {
                let mut v = Vec::new();
                for child in children {
                    v.push(child.display());
                }
                "(".to_string() + &v.join(" ") + ")"
            }
            Val::Boolean(false) => "false".to_string(),
            Val::Boolean(true) => "true".to_string(),
            Val::Function { .. } => "#function".to_string(),
            Val::Primitive(..) => "#primitive".to_string(),
        }
    }

    pub fn as_host_number(&self) -> Result<i32, RuntimeError> {
        match self {
            &Val::Number(i) => Ok(i),
            _ => Err(RuntimeError::UnexpectedValue(
                "number".to_string(),
                self.clone(),
                Loc::Unknown,
            )),
        }
    }

    pub fn as_host_boolean(&self) -> Result<bool, RuntimeError> {
        match self {
            &Val::Nil => Ok(false),
            &Val::Boolean(b) => Ok(b),
            _ => Ok(true),
        }
    }
}

impl PartialOrd for Val {
    fn partial_cmp(&self, other: &Val) -> Option<Ordering> {
        use self::Val::*;
        match (self, other) {
            (Number(a), Number(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Node {
    pub value: Val,
    pub loc: Loc,
}

impl Node {
    pub fn new(value: Val, loc: Loc) -> Self {
        Node { value, loc }
    }
}

impl Deref for Node {
    type Target = Val;

    fn deref(&self) -> &Val {
        &self.value
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PrimitiveObj {
    pub name: String,
    pub min_arity: isize,
    pub max_arity: isize,
}
