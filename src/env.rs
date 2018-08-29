use nodes::Node;
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
}
