use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Write;

mod gc;
pub use gc::Gc;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Flt(i64),
    Str(String),
    Array(Rc<RefCell<Vec<Value>>>),
    Bool(bool),
    Map(Rc<RefCell<std::collections::HashMap<String, Value>>>),
    Obj(Rc<RefCell<std::collections::HashMap<String, Value>>>),
    Lambda(Rc<Lambda>),
    Buffer(Rc<RefCell<Vec<u8>>>),
    Object(Rc<RefCell<Object>>),
}

#[derive(Debug)]
pub struct Object {
    pub class: String,
    pub fields: HashMap<String, Value>,
}

#[derive(Debug)]
pub struct Lambda {
    pub params: Vec<crate::ast::Param>,
    pub ret: crate::types::Type,
    pub body: Vec<crate::ast::Stmt>,
}
#[derive(Debug)]
pub struct Runtime {
    pub scopes: Vec<HashMap<String, Value>>,
    pub func_index: HashMap<String, usize>,
    pub class_index: HashMap<String, usize>,
    pub gc: Gc,
}

impl Runtime {
    pub fn new() -> Self { Self { scopes: vec![HashMap::new()], func_index: HashMap::new(), class_index: HashMap::new(), gc: Gc::new() } }
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
    pub fn pop_scope(&mut self) { self.scopes.pop(); self.gc_maybe_collect(); }
    pub fn fmt_env(&self) -> String {
        let mut out = String::new();
        for (si, scope) in self.scopes.iter().enumerate() {
            out.push_str(&format!("scope {}:\n", si));
            for (k, v) in scope {
                out.push_str(&format!("  {} = {}\n", k, crate::interpreter::fmt_value(v, 2)));
            }
        }
        out
    }
    pub fn new_array(&mut self, v: Vec<Value>) -> Value {
        let rc = Rc::new(RefCell::new(v));
        let rc = self.gc.register_array(rc);
        Value::Array(rc)
    }
    pub fn new_map(&mut self) -> Value {
        let rc = Rc::new(RefCell::new(HashMap::new()));
        let rc = self.gc.register_map(rc);
        Value::Map(rc)
    }
    pub fn new_obj_map(&mut self, m: HashMap<String, Value>) -> Value {
        let rc = Rc::new(RefCell::new(m));
        let rc = self.gc.register_obj(rc);
        Value::Obj(rc)
    }
    pub fn new_buffer(&mut self, v: Vec<u8>) -> Value {
        let rc = Rc::new(RefCell::new(v));
        let rc = self.gc.register_buffer(rc);
        Value::Buffer(rc)
    }
    pub fn new_object(&mut self, class: String, fields: HashMap<String, Value>) -> Value {
        let rc = Rc::new(RefCell::new(Object{ class, fields }));
        let rc = self.gc.register_object(rc);
        Value::Object(rc)
    }
    pub fn gc_maybe_collect(&mut self) { if self.gc.should_collect() { self.gc.collect(&self.scopes); } }
    pub fn gc_collect(&mut self) { self.gc.collect(&self.scopes); }
    pub fn emit(&self, out: &mut String, s: &str) {
        let _ = std::io::stdout().write_all(s.as_bytes());
        let _ = std::io::stdout().flush();
    }
}
