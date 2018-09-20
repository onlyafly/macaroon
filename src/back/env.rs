use ast::{Node, Value};
use back::runtime_error::RuntimeError;
use loc::Loc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub type SmartEnv = Rc<RefCell<Env>>;

#[derive(PartialEq, Debug)]
pub struct Env {
    pub name: String,
    pub map: HashMap<String, Node>,
    pub parent: Option<SmartEnv>,
}

impl Env {
    pub fn new(parent: Option<SmartEnv>) -> SmartEnv {
        let e = Env {
            name: "NONAME".to_string(),
            map: HashMap::new(),
            parent,
        };
        Rc::new(RefCell::new(e))
    }

    pub fn define(&mut self, k: &str, v: Node) -> Result<(), RuntimeError> {
        if self.map.contains_key(k) {
            Err(RuntimeError::CannotRedefine(k.to_string(), v.loc))
        } else {
            self.map.insert(k.to_string(), v);
            Ok(())
        }
    }

    pub fn update(&mut self, k: &str, v: Node) -> Result<(), RuntimeError> {
        let kstring = k.to_string();
        if !self.map.contains_key(k) {
            Err(RuntimeError::CannotUpdateUndefinedName(kstring, v.loc))
        } else {
            self.map.insert(kstring, v);
            Ok(())
        }
    }

    #[allow(dead_code)]
    pub fn exists(&mut self, k: &str) -> bool {
        self.map.contains_key(k)
    }

    pub fn get(&self, name: &str) -> Option<Node> {
        match self.map.get(name) {
            Some(node) => Some(node.clone()),
            None => match self.parent {
                Some(ref parent_env) => parent_env.borrow().get(name),
                None => None,
            },
        }
    }

    pub fn remove(&mut self, k: &str) -> Option<Node> {
        let val = self.map.remove(k);
        // Reinsert nil here so that a later update will update the correct hashmap
        self.map
            .insert(k.to_string(), Node::new(Value::Number(0), Loc::Unknown)); //TODO: should be nil
        val
    }
}
