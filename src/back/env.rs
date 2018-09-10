use ast::Node;
use std::collections::HashMap;

pub struct Env {
    pub map: HashMap<String, Node>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, k: String, v: Node) {
        self.map.insert(k, v);
    }

    pub fn exists(&mut self, k: &str) -> bool {
        self.map.contains_key(k)
    }

    pub fn get(&mut self, k: &str) -> Option<&Node> {
        self.map.get(k)
    }

    pub fn remove(&mut self, k: &str) -> Option<Node> {
        self.map.remove(k)
    }
}
