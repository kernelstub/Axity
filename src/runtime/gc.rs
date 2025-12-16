use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};
use crate::runtime::{Value, Object};

#[derive(Debug)]
pub struct Gc {
    arrays: Vec<Weak<RefCell<Vec<Value>>>>,
    maps: Vec<Weak<RefCell<HashMap<String, Value>>>>,
    objs: Vec<Weak<RefCell<HashMap<String, Value>>>>,
    buffers: Vec<Weak<RefCell<Vec<u8>>>>,
    objects: Vec<Weak<RefCell<Object>>>,
    alloc_count: usize,
    threshold: usize,
}

impl Gc {
    pub fn new() -> Self {
        Self {
            arrays: Vec::new(),
            maps: Vec::new(),
            objs: Vec::new(),
            buffers: Vec::new(),
            objects: Vec::new(),
            alloc_count: 0,
            threshold: 1024,
        }
    }
    pub fn register_array(&mut self, rc: Rc<RefCell<Vec<Value>>>) -> Rc<RefCell<Vec<Value>>> {
        self.arrays.push(Rc::downgrade(&rc));
        self.bump();
        rc
    }
    pub fn register_map(&mut self, rc: Rc<RefCell<HashMap<String, Value>>>) -> Rc<RefCell<HashMap<String, Value>>> {
        self.maps.push(Rc::downgrade(&rc));
        self.bump();
        rc
    }
    pub fn register_obj(&mut self, rc: Rc<RefCell<HashMap<String, Value>>>) -> Rc<RefCell<HashMap<String, Value>>> {
        self.objs.push(Rc::downgrade(&rc));
        self.bump();
        rc
    }
    pub fn register_buffer(&mut self, rc: Rc<RefCell<Vec<u8>>>) -> Rc<RefCell<Vec<u8>>> {
        self.buffers.push(Rc::downgrade(&rc));
        self.bump();
        rc
    }
    pub fn register_object(&mut self, rc: Rc<RefCell<Object>>) -> Rc<RefCell<Object>> {
        self.objects.push(Rc::downgrade(&rc));
        self.bump();
        rc
    }
    fn bump(&mut self) { self.alloc_count = self.alloc_count.saturating_add(1); }
    pub fn should_collect(&self) -> bool { self.alloc_count >= self.threshold }
    pub fn collect(&mut self, scopes: &[HashMap<String, Value>]) {
        let mut marks = Marks::new();
        for scope in scopes {
            for v in scope.values() {
                mark_value(v, &mut marks);
            }
        }
        for w in &self.arrays {
            if let Some(rc) = w.upgrade() {
                let p = Rc::as_ptr(&rc);
                if !marks.arrays.contains(&p) {
                    rc.borrow_mut().clear();
                }
            }
        }
        for w in &self.maps {
            if let Some(rc) = w.upgrade() {
                let p = Rc::as_ptr(&rc);
                if !marks.maps.contains(&p) {
                    rc.borrow_mut().clear();
                }
            }
        }
        for w in &self.objs {
            if let Some(rc) = w.upgrade() {
                let p = Rc::as_ptr(&rc);
                if !marks.objs.contains(&p) {
                    rc.borrow_mut().clear();
                }
            }
        }
        for w in &self.buffers {
            if let Some(rc) = w.upgrade() {
                let p = Rc::as_ptr(&rc);
                if !marks.buffers.contains(&p) {
                    rc.borrow_mut().clear();
                }
            }
        }
        for w in &self.objects {
            if let Some(rc) = w.upgrade() {
                let p = Rc::as_ptr(&rc);
                if !marks.objects.contains(&p) {
                    rc.borrow_mut().fields.clear();
                }
            }
        }
        self.arrays.retain(|w| w.upgrade().is_some());
        self.maps.retain(|w| w.upgrade().is_some());
        self.objs.retain(|w| w.upgrade().is_some());
        self.buffers.retain(|w| w.upgrade().is_some());
        self.objects.retain(|w| w.upgrade().is_some());
        self.alloc_count = 0;
    }
}

struct Marks {
    arrays: HashSet<*const RefCell<Vec<Value>>>,
    maps: HashSet<*const RefCell<HashMap<String, Value>>>,
    objs: HashSet<*const RefCell<HashMap<String, Value>>>,
    buffers: HashSet<*const RefCell<Vec<u8>>>,
    objects: HashSet<*const RefCell<Object>>,
}

impl Marks {
    fn new() -> Self {
        Self {
            arrays: HashSet::new(),
            maps: HashSet::new(),
            objs: HashSet::new(),
            buffers: HashSet::new(),
            objects: HashSet::new(),
        }
    }
}

fn mark_value(v: &Value, marks: &mut Marks) {
    match v {
        Value::Array(rc) => {
            let p = Rc::as_ptr(rc);
            if marks.arrays.insert(p) {
                for el in rc.borrow().iter() {
                    mark_value(el, marks);
                }
            }
        }
        Value::Map(rc) => {
            let p = Rc::as_ptr(rc);
            if marks.maps.insert(p) {
                for val in rc.borrow().values() {
                    mark_value(val, marks);
                }
            }
        }
        Value::Obj(rc) => {
            let p = Rc::as_ptr(rc);
            if marks.objs.insert(p) {
                for val in rc.borrow().values() {
                    mark_value(val, marks);
                }
            }
        }
        Value::Object(rc) => {
            let p = Rc::as_ptr(rc);
            if marks.objects.insert(p) {
                for val in rc.borrow().fields.values() {
                    mark_value(val, marks);
                }
            }
        }
        Value::Buffer(rc) => {
            let p = Rc::as_ptr(rc);
            marks.buffers.insert(p);
        }
        Value::Lambda(_) => {}
        Value::Int(_) => {}
        Value::Flt(_) => {}
        Value::Str(_) => {}
        Value::Bool(_) => {}
    }
}
