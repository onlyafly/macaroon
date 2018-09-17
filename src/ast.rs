use back::runtime_error::RuntimeError;
use loc::Loc;
use std::ops::Deref;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum Value {
    Error(String),
    Number(i32),
    Symbol(String),
    Boolean(bool),
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
            &Value::Boolean(false) => "false".to_string(),
            &Value::Boolean(true) => "true".to_string(),
            n => format!("<unrecognized value: {:?}>", n),
        }
    }

    pub fn as_number_value(&self) -> Result<i32, RuntimeError> {
        match self {
            &Value::Number(i) => Ok(i),
            _ => Err(RuntimeError::UnexpectedValue(
                "number".to_string(),
                self.clone(),
                Loc::Unknown,
            )),
        }
    }

    pub fn as_boolean_value(&self) -> Result<bool, RuntimeError> {
        match self {
            &Value::Boolean(b) => Ok(b),
            _ => Err(RuntimeError::UnexpectedValue(
                "boolean".to_string(),
                self.clone(),
                Loc::Unknown,
            )),
        }
    }
}
