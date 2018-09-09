use ast::WrappedNode;
use std::collections::HashMap;

pub struct Env {
    pub map: HashMap<String, WrappedNode>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: String, v: WrappedNode) {
        self.map.insert(k, v);
    }

    pub fn exists(&mut self, k: &str) -> bool {
        self.map.contains_key(k)
    }

    pub fn get(&mut self, k: &str) -> Option<&WrappedNode> {
        self.map.get(k)
    }

    pub fn remove(&mut self, k: &str) -> Option<WrappedNode> {
        self.map.remove(k)
    }
}
