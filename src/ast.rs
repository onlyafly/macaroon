use back::env::SmartEnv;
use back::runtime_error::RuntimeError;
use loc::Loc;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io;
use std::ops::Deref;
use std::rc::Rc;

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
    Writer(WriterObj),
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Val::Nil => write!(f, "nil"),
            Val::Error(ref s) => write!(f, "#error<{}>", s),
            Val::Number(n) => write!(f, "{}", n),
            Val::StringVal(ref s) => write!(f, "\"{}\"", s),
            Val::Character(ref s) => match s.as_ref() {
                "\n" => write!(f, r"\newline"),
                _ => write!(f, r"\{}", s),
            },
            Val::Symbol(ref s) => write!(f, "{}", s),
            Val::List { ref children } => {
                let mut v = Vec::new();
                for child in children {
                    v.push(format!("{}", child.val));
                }
                write!(f, "({})", &v.join(" "))
            }
            Val::Boolean(false) => write!(f, "false"),
            Val::Boolean(true) => write!(f, "true"),
            Val::Function { .. } => write!(f, "#function"),
            Val::Primitive(..) => write!(f, "#primitive"),
            Val::Writer(..) => write!(f, "#writer"),
        }
    }
}

impl Val {
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
    pub val: Val,
    pub loc: Loc,
}

impl Node {
    pub fn new(val: Val, loc: Loc) -> Self {
        Node { val, loc }
    }
}

impl Deref for Node {
    type Target = Val;

    fn deref(&self) -> &Val {
        &self.val
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct PrimitiveObj {
    pub name: String,
    pub min_arity: isize,
    pub max_arity: isize,
}

pub struct WriterObj {
    pub name: String,
    pub host_writer: Rc<RefCell<dyn io::Write>>,
}

impl PartialEq for WriterObj {
    fn eq(&self, other: &WriterObj) -> bool {
        self.name == other.name
    }
}

impl fmt::Debug for WriterObj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#writer")
    }
}

impl Clone for WriterObj {
    fn clone(&self) -> WriterObj {
        WriterObj {
            name: self.name.clone(),
            host_writer: Rc::clone(&self.host_writer),
        }
    }
}
