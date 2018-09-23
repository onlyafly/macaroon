use back::env::SmartEnv;
use back::runtime_error::RuntimeError;
use loc::Loc;
use std::ops::Deref;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Nil,
    Error(String),
    Number(i32),
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

impl Value {
    pub fn display(&self) -> String {
        #[allow(unreachable_patterns)]
        match self {
            &Value::Nil => "nil".to_string(),
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
            &Value::Boolean(false) => "false".to_string(),
            &Value::Boolean(true) => "true".to_string(),
            n => format!("<unrecognized value: {:?}>", n),
        }
    }

    pub fn as_host_number(&self) -> Result<i32, RuntimeError> {
        match self {
            &Value::Number(i) => Ok(i),
            _ => Err(RuntimeError::UnexpectedValue(
                "number".to_string(),
                self.clone(),
                Loc::Unknown,
            )),
        }
    }

    pub fn as_host_boolean(&self) -> Result<bool, RuntimeError> {
        match self {
            &Value::Nil => Ok(false),
            &Value::Boolean(b) => Ok(b),
            &Value::Number(_) => Ok(true),
            &Value::List { .. } => Ok(true),
            _ => Err(RuntimeError::UnexpectedValue(
                "boolean".to_string(),
                self.clone(),
                Loc::Unknown,
            )),
        }
    }
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

#[derive(PartialEq, Debug, Clone)]
pub struct PrimitiveObj {
    pub name: String,
    pub min_arity: isize,
    pub max_arity: isize,
}
