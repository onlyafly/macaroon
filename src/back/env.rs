use ast::{Node, Value};
use back::runtime_error::RuntimeError;
use loc::Loc;
use std::collections::HashMap;

pub struct Env {
    pub map: HashMap<String, Node>,
    pub parent: Option<Box<Env>>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            map: HashMap::new(),
            parent: None,
        }
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

    pub fn get(&mut self, k: &str) -> Option<&Node> {
        self.map.get(k)
    }

    pub fn remove(&mut self, k: &str) -> Option<Node> {
        let val = self.map.remove(k);
        // Reinsert nil here so that a later update will update the correct hashmap
        self.map
            .insert(k.to_string(), Node::new(Value::Number(0), Loc::Unknown)); //TODO: should be nil
        val
    }
}
