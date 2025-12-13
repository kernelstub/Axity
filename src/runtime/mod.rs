use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Value { Int(i64) }

#[derive(Debug)]
pub struct Runtime {
    pub scopes: Vec<HashMap<String, Value>>,
}

impl Runtime {
    pub fn new() -> Self { Self { scopes: vec![HashMap::new()] } }
    pub fn get(&self, name: &str) -> Option<Value> {
        for i in (0..self.scopes.len()).rev() { if let Some(v) = self.scopes[i].get(name) { return Some(v.clone()); } }
        None
    }
    pub fn set(&mut self, name: String, v: Value) { if let Some(m) = self.scopes.last_mut() { m.insert(name, v); } }
    pub fn assign(&mut self, name: &str, v: Value) -> bool {
        for i in (0..self.scopes.len()).rev() { if self.scopes[i].contains_key(name) { self.scopes[i].insert(name.to_string(), v); return true; } }
        false
    }
    pub fn push_scope(&mut self) { self.scopes.push(HashMap::new()); }
    pub fn pop_scope(&mut self) { self.scopes.pop(); }
}

