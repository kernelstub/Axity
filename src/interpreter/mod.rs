use crate::ast::*;
use crate::error::AxityError;
use crate::runtime::{Runtime, Value, Object};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

pub fn execute(p: &Program, rt: &mut Runtime, out: &mut String) -> Result<(), AxityError> {
    for it in &p.items { if let Item::Stmt(s) = it { exec_stmt(p, s, rt, out)?; } }
    let has_main = p.items.iter().any(|it| if let Item::Func(f)=it { f.name=="main" } else { false });
    if has_main { call_func("main", &[], p, rt, out)?; }
    Ok(())
}

fn exec_stmt(p: &Program, s: &Stmt, rt: &mut Runtime, out: &mut String) -> Result<Control, AxityError> {
    match s {
        Stmt::Let{ name, init, .. } => { let v = eval_expr(p, init, rt, out)?; rt.set(name.clone(), v); Ok(Control::Next) }
        Stmt::Assign{ name, expr, .. } => { let v = eval_expr(p, expr, rt, out)?; if !rt.assign(name, v) { return Err(AxityError::rt("assign to undefined variable")); } Ok(Control::Next) }
        Stmt::Print{ expr, .. } => {
            let v = eval_expr(p, expr, rt, out)?;
            let s = fmt_value(&v, 2);
            out.push_str(&s);
            out.push('\n');
            Ok(Control::Next)
        }
        Stmt::Expr(e) => { let _ = eval_expr(p, e, rt, out)?; Ok(Control::Next) }
        Stmt::MemberAssign{ object, field, expr, .. } => {
            let ov = eval_expr(p, object, rt, out)?;
            match ov {
                Value::Object(rc) => {
                    // Specialized evaluation for numeric field updates
                    if let Expr::Binary{ op, left: _, right, .. } = expr {
                        let cur = rc.borrow().fields.get(field).cloned().unwrap_or(Value::Int(0));
                        let rhs = eval_expr(p, right, rt, out)?;
                        if let Value::Int(ci) = cur {
                            let ri = match rhs {
                                Value::Int(i) => i,
                                Value::Bool(b) => if b {1} else {0},
                                _ => { let v = eval_expr(p, expr, rt, out)?; rc.borrow_mut().fields.insert(field.clone(), v); return Ok(Control::Next) }
                            };
                            let nv = match op {
                                BinOp::Add => ci + ri,
                                BinOp::Sub => ci - ri,
                                BinOp::Mul => ci * ri,
                                BinOp::Div => ci / ri,
                                BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge | BinOp::Eq | BinOp::Ne => if ci == ri {1} else {0},
                                BinOp::And | BinOp::Or => if ci != 0 && ri != 0 {1} else {0},
                            };
                            rc.borrow_mut().fields.insert(field.clone(), Value::Int(nv));
                            return Ok(Control::Next);
                        }
                    }
                    let v = eval_expr(p, expr, rt, out)?;
                    rc.borrow_mut().fields.insert(field.clone(), v);
                    Ok(Control::Next)
                }
                _ => Err(AxityError::rt("member assignment on non-object"))
            }
        }
        Stmt::While{ cond, body, .. } => {
            loop {
                let c = eval_expr(p, cond, rt, out)?;
                let ci = match c { Value::Int(i) => i, Value::Bool(b) => if b {1} else {0}, _ => 0 };
                if ci == 0 { break; }
                rt.push_scope();
                for st in body { if let Control::Return(_) = exec_stmt(p, st, rt, out)? { rt.pop_scope(); return Ok(Control::Next); } }
                rt.pop_scope();
            }
            Ok(Control::Next)
        }
        Stmt::If{ cond, then_body, else_body, .. } => {
            let c = eval_expr(p, cond, rt, out)?;
            let ci = match c { Value::Int(i) => i, Value::Bool(b) => if b {1} else {0}, _ => 0 };
            rt.push_scope();
            if ci != 0 {
                for st in then_body { if let Control::Return(_) = exec_stmt(p, st, rt, out)? { rt.pop_scope(); return Ok(Control::Next); } }
            } else {
                for st in else_body { if let Control::Return(_) = exec_stmt(p, st, rt, out)? { rt.pop_scope(); return Ok(Control::Next); } }
            }
            rt.pop_scope();
            Ok(Control::Next)
        }
        Stmt::Return{ expr, .. } => { let v = eval_expr(p, expr, rt, out)?; Ok(Control::Return(v)) }
        Stmt::Match{ expr, arms, default, .. } => {
            let v = eval_expr(p, expr, rt, out)?;
            let mut matched = false;
            for arm in arms {
                let ok = match (&arm.pat, &v) {
                    (Pattern::PInt(pi), Value::Int(vi)) => *pi == *vi,
                    (Pattern::PStr(ps), Value::Str(vs)) => *ps == *vs,
                    (Pattern::PBool(pb), Value::Bool(vb)) => *pb == *vb,
                    _ => false,
                };
                if ok {
                    matched = true;
                    rt.push_scope();
                    for st in &arm.body { if let Control::Return(_) = exec_stmt(p, st, rt, out)? { rt.pop_scope(); return Ok(Control::Next); } }
                    rt.pop_scope();
                    break;
                }
            }
            if !matched {
                if let Some(body) = default {
                    rt.push_scope();
                    for st in body { if let Control::Return(_) = exec_stmt(p, st, rt, out)? { rt.pop_scope(); return Ok(Control::Next); } }
                    rt.pop_scope();
                }
            }
            Ok(Control::Next)
        }
    }
}

fn eval_expr(p: &Program, e: &Expr, rt: &mut Runtime, out: &mut String) -> Result<Value, AxityError> {
    match e {
        Expr::Int(i, _) => Ok(Value::Int(*i)),
        Expr::Var(name, _) => rt.get(name).ok_or_else(|| AxityError::rt("read of undefined variable")),
        Expr::Str(s, _) => Ok(Value::Str(s.clone())),
        Expr::ArrayLit(elems, _) => {
            let mut v = Vec::new();
            for el in elems { v.push(eval_expr(p, el, rt, out)?); }
            Ok(Value::Array(Rc::new(RefCell::new(v))))
        }
        Expr::Bool(b, _) => Ok(Value::Bool(*b)),
        Expr::Binary{ op, left, right, .. } => {
            let l = eval_expr(p, left, rt, out)?; let r = eval_expr(p, right, rt, out)?;
            match (l, r) {
                (Value::Int(li), Value::Int(ri)) => {
                    let v = match op {
                        BinOp::Add => li + ri,
                        BinOp::Sub => li - ri,
                        BinOp::Mul => li * ri,
                        BinOp::Div => li / ri,
                        BinOp::Lt => if li < ri {1} else {0},
                        BinOp::Le => if li <= ri {1} else {0},
                        BinOp::Gt => if li > ri {1} else {0},
                        BinOp::Ge => if li >= ri {1} else {0},
                        BinOp::Eq => if li == ri {1} else {0},
                        BinOp::Ne => if li != ri {1} else {0},
                        BinOp::And | BinOp::Or => return Err(AxityError::rt("logical on ints")),
                    };
                    Ok(Value::Int(v))
                }
                (Value::Int(li), Value::Bool(rb)) | (Value::Bool(rb), Value::Int(li)) => {
                    let ri = if rb { 1 } else { 0 };
                    let v = match op {
                        BinOp::Add => li + ri,
                        BinOp::Sub => li - ri,
                        BinOp::Mul => li * ri,
                        BinOp::Div => li / ri,
                        BinOp::Lt => if li < ri {1} else {0},
                        BinOp::Le => if li <= ri {1} else {0},
                        BinOp::Gt => if li > ri {1} else {0},
                        BinOp::Ge => if li >= ri {1} else {0},
                        BinOp::Eq => if li == ri {1} else {0},
                        BinOp::Ne => if li != ri {1} else {0},
                        BinOp::And | BinOp::Or => return Err(AxityError::rt("unsupported bool op")),
                    };
                    Ok(Value::Int(v))
                }
                (Value::Str(ls), Value::Str(rs)) => {
                    match op {
                        BinOp::Add => Ok(Value::Str(format!("{}{}", ls, rs))),
                        BinOp::Eq => Ok(Value::Int(if ls == rs {1} else {0})),
                        BinOp::Ne => Ok(Value::Int(if ls != rs {1} else {0})),
                        _ => Err(AxityError::rt("unsupported string binary op"))
                    }
                }
                (Value::Int(li), Value::Object(_)) | (Value::Object(_), Value::Int(li)) => {
                    let ri = 0;
                    let v = match op {
                        BinOp::Add => li + ri,
                        BinOp::Sub => li - ri,
                        BinOp::Mul => li * ri,
                        BinOp::Div => if ri==0 { li } else { li / ri },
                        BinOp::Lt => if li < ri {1} else {0},
                        BinOp::Le => if li <= ri {1} else {0},
                        BinOp::Gt => if li > ri {1} else {0},
                        BinOp::Ge => if li >= ri {1} else {0},
                        BinOp::Eq => if li == ri {1} else {0},
                        BinOp::Ne => if li != ri {1} else {0},
                        BinOp::And | BinOp::Or => return Err(AxityError::rt("unsupported bool op")),
                    };
                    Ok(Value::Int(v))
                }
                (Value::Bool(lb), Value::Bool(rb)) => {
                    let v = match op {
                        BinOp::And => lb && rb,
                        BinOp::Or => lb || rb,
                        BinOp::Eq => lb == rb,
                        BinOp::Ne => lb != rb,
                        _ => return Err(AxityError::rt("unsupported bool op")),
                    };
                    Ok(match op { BinOp::And | BinOp::Or => Value::Bool(v), _ => Value::Int(if v {1} else {0}) })
                }
                _ => Err(AxityError::rt("type mismatch in binary"))
            }
        }
        Expr::UnaryNot{ expr, .. } => {
            let v = eval_expr(p, expr, rt, out)?;
            match v {
                Value::Bool(b) => Ok(Value::Bool(!b)),
                _ => Err(AxityError::rt("! requires bool"))
            }
        }
        Expr::New(name, args, _) => {
            let mut fields = std::collections::HashMap::new();
            for it in &p.items {
                if let Item::Class(c) = it {
                    if c.name.as_str() == name.as_str() {
                        for f in &c.fields {
                            let dv = match f.ty {
                                crate::types::Type::Int => Value::Int(0),
                                crate::types::Type::String => Value::Str(String::new()),
                                crate::types::Type::Array(_) => Value::Array(Rc::new(RefCell::new(Vec::new()))),
                                crate::types::Type::Class(_) => Value::Object(Rc::new(RefCell::new(Object{ class: String::new(), fields: HashMap::new() }))),
                                crate::types::Type::Bool => Value::Bool(false),
                                crate::types::Type::Map(_) => Value::Map(Rc::new(RefCell::new(HashMap::new()))),
                            };
                            fields.insert(f.name.clone(), dv);
                        }
                        let obj = Rc::new(RefCell::new(Object{ class: name.clone(), fields }));
                        // call init if present
                        if c.methods.iter().any(|m| m.name == "init") {
                            let mut ev_args = Vec::new();
                            ev_args.push(Value::Object(obj.clone()));
                            for a in args { ev_args.push(eval_expr(p, a, rt, out)?); }
                            let _ = call_method("init", &ev_args, p, rt, out)?;
                        }
                        return Ok(Value::Object(obj));
                    }
                }
            }
            Ok(Value::Object(Rc::new(RefCell::new(Object{ class: name.clone(), fields }))))
        }
        Expr::Member{ object, field, .. } => {
            let ov = eval_expr(p, object, rt, out)?;
            match ov {
                Value::Object(rc) => {
                    let b = rc.borrow();
                    b.fields.get(field).cloned().ok_or_else(|| AxityError::rt("unknown field"))
                }
                _ => Err(AxityError::rt("member access on non-object"))
            }
        }
        Expr::Index{ array, index, .. } => {
            let av = eval_expr(p, array, rt, out)?;
            let iv = eval_expr(p, index, rt, out)?;
            let idx = match iv { Value::Int(i) => i as usize, _ => return Err(AxityError::rt("index non-int")) };
            match av {
                Value::Array(vs) => {
                    let vsb = vs.borrow();
                    if idx >= vsb.len() { return Err(AxityError::rt("index out of bounds")); }
                    Ok(vsb[idx].clone())
                }
                _ => Err(AxityError::rt("index on non-array"))
            }
        }
        Expr::MethodCall{ object, name, args, .. } => {
            let ov = eval_expr(p, object, rt, out)?;
            let mut ev_args = Vec::new();
            ev_args.push(ov.clone());
            for a in args { ev_args.push(eval_expr(p, a, rt, out)?); }
            call_method(name, &ev_args, p, rt, out)
        }
        Expr::Call{ name, args, .. } => {
            if name == "len" {
                if args.len() != 1 { return Err(AxityError::rt("len expects one argument")); }
                let av = eval_expr(p, &args[0], rt, out)?;
                match av {
                    Value::Array(v) => Ok(Value::Int(v.borrow().len() as i64)),
                    Value::Str(s) => Ok(Value::Int(s.len() as i64)),
                    _ => Err(AxityError::rt("len expects array or string"))
                }
            } else if name == "slice" {
                if args.len() != 3 { return Err(AxityError::rt("slice expects (array, start, len)")); }
                let arr = eval_expr(p, &args[0], rt, out)?;
                let st = match eval_expr(p, &args[1], rt, out)? { Value::Int(i) => i as usize, _ => return Err(AxityError::rt("start must be int")) };
                let ln = match eval_expr(p, &args[2], rt, out)? { Value::Int(i) => i as usize, _ => return Err(AxityError::rt("len must be int")) };
                match arr {
                    Value::Array(v) => {
                        let vb = v.borrow();
                        let end = st.saturating_add(ln).min(vb.len());
                        let mut outv = Vec::new();
                        for i in st..end { outv.push(vb[i].clone()); }
                        Ok(Value::Array(Rc::new(RefCell::new(outv))))
                    }
                    _ => Err(AxityError::rt("slice expects array"))
                }
            } else if name == "range" {
                if args.len() != 2 { return Err(AxityError::rt("range expects (start, end)")); }
                let st = match eval_expr(p, &args[0], rt, out)? { Value::Int(i) => i, _ => return Err(AxityError::rt("start must be int")) };
                let en = match eval_expr(p, &args[1], rt, out)? { Value::Int(i) => i, _ => return Err(AxityError::rt("end must be int")) };
                let step = if en >= st { 1 } else { -1 };
                let mut v = Vec::new();
                let mut i = st;
                while (step > 0 && i < en) || (step < 0 && i > en) { v.push(Value::Int(i)); i += step; }
                Ok(Value::Array(Rc::new(RefCell::new(v))))
            } else if name == "map_remove" {
                if args.len() != 2 { return Err(AxityError::rt("map_remove expects (map, key)")); }
                let m = eval_expr(p, &args[0], rt, out)?;
                let k = match eval_expr(p, &args[1], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("key must be string")) };
                match m {
                    Value::Map(mm) => Ok(Value::Int(if mm.borrow_mut().remove(&k).is_some() {1} else {0})),
                    _ => Err(AxityError::rt("first arg must be map"))
                }
            } else if name == "map_clear" {
                if args.len() != 1 { return Err(AxityError::rt("map_clear expects (map)")); }
                let m = eval_expr(p, &args[0], rt, out)?;
                match m {
                    Value::Map(mm) => { let sz = mm.borrow().len() as i64; mm.borrow_mut().clear(); Ok(Value::Int(sz)) }
                    _ => Err(AxityError::rt("first arg must be map"))
                }
            } else if name == "map_size" {
                if args.len() != 1 { return Err(AxityError::rt("map_size expects (map)")); }
                let m = eval_expr(p, &args[0], rt, out)?;
                match m {
                    Value::Map(mm) => Ok(Value::Int(mm.borrow().len() as i64)),
                    _ => Err(AxityError::rt("first arg must be map"))
                }
            } else if name == "string_replace" {
                if args.len() != 3 { return Err(AxityError::rt("string_replace expects (s, from, to)")); }
                let s = match eval_expr(p, &args[0], rt, out)? { Value::Str(v) => v, _ => return Err(AxityError::rt("s must be string")) };
                let from = match eval_expr(p, &args[1], rt, out)? { Value::Str(v) => v, _ => return Err(AxityError::rt("from must be string")) };
                let to = match eval_expr(p, &args[2], rt, out)? { Value::Str(v) => v, _ => return Err(AxityError::rt("to must be string")) };
                Ok(Value::Str(s.replace(&from, &to)))
            } else if name == "string_split" {
                if args.len() != 2 { return Err(AxityError::rt("string_split expects (s, sep)")); }
                let s = match eval_expr(p, &args[0], rt, out)? { Value::Str(v) => v, _ => return Err(AxityError::rt("s must be string")) };
                let sep = match eval_expr(p, &args[1], rt, out)? { Value::Str(v) => v, _ => return Err(AxityError::rt("sep must be string")) };
                let mut v = Vec::new();
                for part in s.split(&sep) { v.push(Value::Str(part.to_string())); }
                Ok(Value::Array(Rc::new(RefCell::new(v))))
            } else if name == "push" {
                if args.len() != 2 { return Err(AxityError::rt("push expects array and value")); }
                let arr = eval_expr(p, &args[0], rt, out)?;
                let val = eval_expr(p, &args[1], rt, out)?;
                match arr {
                    Value::Array(v) => { v.borrow_mut().push(val); Ok(Value::Int(v.borrow().len() as i64)) }
                    _ => Err(AxityError::rt("push expects array"))
                }
            } else if name == "pop" {
                if args.len() != 1 { return Err(AxityError::rt("pop expects array")); }
                let arr = eval_expr(p, &args[0], rt, out)?;
                match arr {
                    Value::Array(v) => { v.borrow_mut().pop().ok_or_else(|| AxityError::rt("pop from empty array")) }
                    _ => Err(AxityError::rt("pop expects array"))
                }
            } else if name == "set" {
                if args.len() != 3 { return Err(AxityError::rt("set expects array, index, value")); }
                let arr = eval_expr(p, &args[0], rt, out)?;
                let idxv = eval_expr(p, &args[1], rt, out)?;
                let val = eval_expr(p, &args[2], rt, out)?;
                let idx = match idxv { Value::Int(i) => i as usize, _ => return Err(AxityError::rt("set index must be int")) };
                match arr {
                    Value::Array(v) => { let mut vb = v.borrow_mut(); if idx>=vb.len() { return Err(AxityError::rt("index out of bounds")); } vb[idx] = val; Ok(Value::Int(idx as i64)) }
                    _ => Err(AxityError::rt("set expects array"))
                }
            } else if name == "map_new_int" {
                if args.len() != 0 { return Err(AxityError::rt("map_new_int expects no args")); }
                Ok(Value::Map(Rc::new(RefCell::new(std::collections::HashMap::new()))))
            } else if name == "map_new_string" {
                if args.len() != 0 { return Err(AxityError::rt("map_new_string expects no args")); }
                Ok(Value::Map(Rc::new(RefCell::new(std::collections::HashMap::new()))))
            } else if name == "map_set" {
                let m = eval_expr(p, &args[0], rt, out)?;
                let k = eval_expr(p, &args[1], rt, out)?;
                let v = eval_expr(p, &args[2], rt, out)?;
                let key = match k { Value::Str(s) => s, _ => return Err(AxityError::rt("map key must be string")) };
                match m {
                    Value::Map(mm) => { mm.borrow_mut().insert(key, v); Ok(Value::Int(1)) }
                    _ => Err(AxityError::rt("first arg must be map"))
                }
            } else if name == "map_get" {
                let m = eval_expr(p, &args[0], rt, out)?;
                let k = eval_expr(p, &args[1], rt, out)?;
                let key = match k { Value::Str(s) => s, _ => return Err(AxityError::rt("map key must be string")) };
                match m {
                    Value::Map(mm) => Ok(mm.borrow().get(&key).cloned().unwrap_or(Value::Int(0))),
                    _ => Err(AxityError::rt("first arg must be map"))
                }
            } else if name == "map_has" {
                let m = eval_expr(p, &args[0], rt, out)?;
                let k = eval_expr(p, &args[1], rt, out)?;
                let key = match k { Value::Str(s) => s, _ => return Err(AxityError::rt("map key must be string")) };
                match m {
                    Value::Map(mm) => Ok(Value::Bool(mm.borrow().contains_key(&key))),
                    _ => Err(AxityError::rt("first arg must be map"))
                }
            } else if name == "map_keys" {
                let m = eval_expr(p, &args[0], rt, out)?;
                match m {
                    Value::Map(mm) => {
                        let mut v = Vec::new();
                        for k in mm.borrow().keys() { v.push(Value::Str(k.clone())); }
                        Ok(Value::Array(Rc::new(RefCell::new(v))))
                    }
                    _ => Err(AxityError::rt("first arg must be map"))
                }
            } else if name == "read_file" {
                if args.len() != 1 { return Err(AxityError::rt("read_file expects path")); }
                let pth = match eval_expr(p, &args[0], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("path must be string")) };
                match std::fs::read_to_string(&pth) { Ok(s) => Ok(Value::Str(s)), Err(e) => Err(AxityError::rt(&format!("read error: {}", e))) }
            } else if name == "write_file" {
                if args.len() != 2 { return Err(AxityError::rt("write_file expects (path, content)")); }
                let pth = match eval_expr(p, &args[0], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("path must be string")) };
                let content = match eval_expr(p, &args[1], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("content must be string")) };
                match std::fs::write(&pth, content) { Ok(_) => Ok(Value::Int(1)), Err(e) => Err(AxityError::rt(&format!("write error: {}", e))) }
            } else if name == "mkdir" {
                if args.len() != 1 { return Err(AxityError::rt("mkdir expects path")); }
                let pth = match eval_expr(p, &args[0], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("path must be string")) };
                match std::fs::create_dir_all(&pth) { Ok(_) => Ok(Value::Int(1)), Err(e) => Err(AxityError::rt(&format!("mkdir error: {}", e))) }
            } else if name == "exists" {
                if args.len() != 1 { return Err(AxityError::rt("exists expects path")); }
                let pth = match eval_expr(p, &args[0], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("path must be string")) };
                Ok(Value::Bool(std::path::Path::new(&pth).exists()))
            } else if name == "read_json" {
                let pth = match eval_expr(p, &args[0], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("path must be string")) };
                match std::fs::read_to_string(&pth) { Ok(s) => Ok(Value::Str(s)), Err(e) => Err(AxityError::rt(&format!("read error: {}", e))) }
            } else if name == "write_json" {
                let pth = match eval_expr(p, &args[0], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("path must be string")) };
                let content = match eval_expr(p, &args[1], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("content must be string")) };
                if serde_json::from_str::<serde_json::Value>(&content).is_err() { return Err(AxityError::rt("invalid json")) }
                match std::fs::write(&pth, content) { Ok(_) => Ok(Value::Int(1)), Err(e) => Err(AxityError::rt(&format!("write error: {}", e))) }
            } else if name == "json_get" {
                if args.len() != 2 { return Err(AxityError::rt("json_get expects (json, key)")); }
                let content = match eval_expr(p, &args[0], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("json must be string")) };
                let key = match eval_expr(p, &args[1], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("key must be string")) };
                let v: serde_json::Value = serde_json::from_str(&content).map_err(|e| AxityError::rt(&format!("json parse: {}", e)))?;
                let res = v.get(&key).cloned().unwrap_or(serde_json::Value::Null);
                Ok(Value::Str(res.to_string()))
            } else if name == "json_set" {
                if args.len() != 3 { return Err(AxityError::rt("json_set expects (json, key, value)")); }
                let content = match eval_expr(p, &args[0], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("json must be string")) };
                let key = match eval_expr(p, &args[1], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("key must be string")) };
                let value = match eval_expr(p, &args[2], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("value must be string")) };
                let mut v: serde_json::Value = serde_json::from_str(&content).map_err(|e| AxityError::rt(&format!("json parse: {}", e)))?;
                if let serde_json::Value::Object(ref mut m) = v {
                    m.insert(key, serde_json::Value::String(value));
                    Ok(Value::Str(v.to_string()))
                } else { Err(AxityError::rt("json_set requires object at root")) }
            } else if name == "read_toml" {
                let pth = match eval_expr(p, &args[0], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("path must be string")) };
                match std::fs::read_to_string(&pth) { Ok(s) => Ok(Value::Str(s)), Err(e) => Err(AxityError::rt(&format!("read error: {}", e))) }
            } else if name == "write_toml" {
                let pth = match eval_expr(p, &args[0], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("path must be string")) };
                let content = match eval_expr(p, &args[1], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("content must be string")) };
                match std::fs::write(&pth, content) { Ok(_) => Ok(Value::Int(1)), Err(e) => Err(AxityError::rt(&format!("write error: {}", e))) }
            } else if name == "toml_get" {
                if args.len() != 2 { return Err(AxityError::rt("toml_get expects (toml, key.path)")); }
                let content = match eval_expr(p, &args[0], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("toml must be string")) };
                let key = match eval_expr(p, &args[1], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("key must be string")) };
                let parts: Vec<&str> = key.split('.').collect();
                let mut section: Option<&str> = None;
                let mut field: &str = parts[0];
                if parts.len() == 2 { section = Some(parts[0]); field = parts[1]; }
                let mut in_section = section.is_none();
                for line in content.lines() {
                    let l = line.trim();
                    if l.starts_with('[') && l.ends_with(']') {
                        in_section = match section { Some(sec) => &l[1..l.len()-1] == sec, None => false };
                        continue;
                    }
                    if !in_section { continue; }
                    if let Some((k,v)) = l.split_once('=') {
                        if k.trim() == field { return Ok(Value::Str(v.trim().trim_matches('"').to_string())); }
                    }
                }
                Ok(Value::Str(String::new()))
            } else if name == "toml_set" {
                if args.len() != 3 { return Err(AxityError::rt("toml_set expects (toml, key.path, value)")); }
                let content = match eval_expr(p, &args[0], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("toml must be string")) };
                let key = match eval_expr(p, &args[1], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("key must be string")) };
                let value = match eval_expr(p, &args[2], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("value must be string")) };
                let parts: Vec<&str> = key.split('.').collect();
                let mut lines: Vec<String> = Vec::new();
                let mut wrote = false;
                let mut in_section = parts.len()==1;
                let mut _section_written = false;
                let (section, field) = if parts.len()==2 { (Some(parts[0]), parts[1]) } else { (None, parts[0]) };
                for line in content.lines() {
                    let l = line.trim();
                    if l.starts_with('[') && l.ends_with(']') {
                        in_section = match section { Some(sec) => &l[1..l.len()-1] == sec, None => false };
                        lines.push(line.to_string());
                        continue;
                    }
                    if in_section {
                        if let Some((k,_)) = l.split_once('=') {
                            if k.trim() == field {
                                lines.push(format!("{} = \"{}\"", field, value));
                                wrote = true;
                                continue;
                            }
                        }
                    }
                    lines.push(line.to_string());
                }
                if !wrote {
                    if let Some(sec) = section {
                        if !content.contains(&format!("[{}]", sec)) && !_section_written {
                            lines.push(format!("[{}]", sec));
                            _section_written = true;
                        }
                        lines.push(format!("{} = \"{}\"", field, value));
                    } else {
                        lines.push(format!("{} = \"{}\"", field, value));
                    }
                }
                Ok(Value::Str(lines.join("\n")))
            } else if name == "read_env" {
                let pth = match eval_expr(p, &args[0], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("path must be string")) };
                match std::fs::read_to_string(&pth) { Ok(s) => Ok(Value::Str(s)), Err(e) => Err(AxityError::rt(&format!("read error: {}", e))) }
            } else if name == "write_env" {
                let pth = match eval_expr(p, &args[0], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("path must be string")) };
                let content = match eval_expr(p, &args[1], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("content must be string")) };
                match std::fs::write(&pth, content) { Ok(_) => Ok(Value::Int(1)), Err(e) => Err(AxityError::rt(&format!("write error: {}", e))) }
            } else if name == "env_get" {
                if args.len() != 2 { return Err(AxityError::rt("env_get expects (file_content, key)")); }
                let content = match eval_expr(p, &args[0], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("content must be string")) };
                let key = match eval_expr(p, &args[1], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("key must be string")) };
                for line in content.lines() {
                    if let Some((k,v)) = line.split_once('=') { if k.trim()==key { return Ok(Value::Str(v.trim().to_string())); } }
                }
                Ok(Value::Str(String::new()))
            } else if name == "env_set" {
                if args.len() != 3 { return Err(AxityError::rt("env_set expects (file_content, key, value)")); }
                let content = match eval_expr(p, &args[0], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("content must be string")) };
                let key = match eval_expr(p, &args[1], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("key must be string")) };
                let value = match eval_expr(p, &args[2], rt, out)? { Value::Str(s) => s, _ => return Err(AxityError::rt("value must be string")) };
                let mut lines: Vec<String> = Vec::new(); let mut found=false;
                for line in content.lines() {
                    if let Some((k,_)) = line.split_once('=') {
                        if k.trim()==key { lines.push(format!("{}={}", key, value)); found=true; } else { lines.push(line.to_string()); }
                    } else { lines.push(line.to_string()); }
                }
                if !found { lines.push(format!("{}={}", key, value)); }
                Ok(Value::Str(lines.join("\n")))
            } else if name == "strlen" {
                if args.len() != 1 { return Err(AxityError::rt("strlen expects one argument")); }
                let s = eval_expr(p, &args[0], rt, out)?;
                match s { Value::Str(ss) => Ok(Value::Int(ss.len() as i64)), _ => Err(AxityError::rt("strlen expects string")) }
            } else if name == "substr" {
                if args.len() != 3 { return Err(AxityError::rt("substr expects (string, start, len)")); }
                let s = eval_expr(p, &args[0], rt, out)?; let st = eval_expr(p, &args[1], rt, out)?; let ln = eval_expr(p, &args[2], rt, out)?;
                let start = match st { Value::Int(i) => i as usize, _ => return Err(AxityError::rt("substr start must be int")) };
                let len = match ln { Value::Int(i) => i as usize, _ => return Err(AxityError::rt("substr len must be int")) };
                match s { Value::Str(ss) => {
                    let end = start.saturating_add(len).min(ss.len());
                    Ok(Value::Str(ss[start.min(ss.len())..end].to_string()))
                }, _ => Err(AxityError::rt("substr expects string")) }
            } else if name == "index_of" {
                if args.len() != 2 { return Err(AxityError::rt("index_of expects (string, string)")); }
                let s = eval_expr(p, &args[0], rt, out)?; let sub = eval_expr(p, &args[1], rt, out)?;
                match (s, sub) { (Value::Str(ss), Value::Str(subs)) => Ok(Value::Int(ss.find(&subs).map(|i| i as i64).unwrap_or(-1))), _ => Err(AxityError::rt("index_of expects strings")) }
            } else if name == "to_int" {
                if args.len() != 1 { return Err(AxityError::rt("to_int expects one argument")); }
                let s = eval_expr(p, &args[0], rt, out)?;
                match s { Value::Str(ss) => Ok(Value::Int(ss.parse::<i64>().unwrap_or(0))), _ => Err(AxityError::rt("to_int expects string")) }
            } else if name == "to_string" {
                if args.len() != 1 { return Err(AxityError::rt("to_string expects one argument")); }
                let i = eval_expr(p, &args[0], rt, out)?;
                match i { Value::Int(ii) => Ok(Value::Str(ii.to_string())), _ => Err(AxityError::rt("to_string expects int")) }
            } else {
                let mut ev_args = Vec::new();
                for a in args { ev_args.push(eval_expr(p, a, rt, out)?); }
                call_func(name, &ev_args, p, rt, out)
            }
        }
    }
}

fn call_func(name: &str, args: &[Value], p: &Program, rt: &mut Runtime, out: &mut String) -> Result<Value, AxityError> {
    let f = p.items.iter().find_map(|it| if let Item::Func(f)=it { if f.name==name { Some(f) } else { None } } else { None }).ok_or_else(|| AxityError::rt("undefined function"))?;
    rt.push_scope();
    for (i,par) in f.params.iter().enumerate() { rt.set(par.name.clone(), args.get(i).cloned().unwrap_or(Value::Int(0))); }
    for st in &f.body { match exec_stmt(p, st, rt, out)? { Control::Next => {}, Control::Return(v) => { rt.pop_scope(); return Ok(v); } } }
    rt.pop_scope();
    Ok(Value::Int(0))
}

fn call_method(name: &str, args: &[Value], p: &Program, rt: &mut Runtime, out: &mut String) -> Result<Value, AxityError> {
    let (obj, rest) = args.split_first().ok_or_else(|| AxityError::rt("missing receiver"))?;
    let class_name = match obj { Value::Object(rc) => rc.borrow().class.clone(), _ => return Err(AxityError::rt("receiver is not object")) };
    let f = p.items.iter().find_map(|it| if let Item::Class(c)=it {
        if c.name == class_name {
            for m in &c.methods { if m.name == name { return Some(m); } }
        }
        None
    } else { None }).ok_or_else(|| AxityError::rt("undefined method"))?;
    rt.push_scope();
    for (i,par) in f.params.iter().enumerate() { rt.set(par.name.clone(), if i==0 { args.get(0).cloned().unwrap() } else { rest.get(i-1).cloned().unwrap_or(Value::Int(0)) }); }
    for st in &f.body { match exec_stmt(p, st, rt, out)? { Control::Next => {}, Control::Return(v) => { rt.pop_scope(); return Ok(v); } } }
    rt.pop_scope();
    Ok(Value::Int(0))
}

enum Control { Next, Return(Value) }

pub fn fmt_value(v: &Value, depth: usize) -> String {
    if depth == 0 { return String::from("..."); }
    match v {
        Value::Int(i) => i.to_string(),
        Value::Str(s) => s.clone(),
        Value::Bool(b) => if *b { "true".to_string() } else { "false".to_string() },
        Value::Array(a) => {
            let ab = a.borrow();
            let mut parts = Vec::new();
            for el in ab.iter() { parts.push(fmt_value(el, depth-1)); }
            format!("[{}]", parts.join(", "))
        }
        Value::Map(m) => {
            let mut parts = Vec::new();
            for (k, val) in m.borrow().iter() { parts.push(format!("{}: {}", k, fmt_value(val, depth-1))); }
            format!("{{{}}}", parts.join(", "))
        }
        Value::Object(rc) => {
            let b = rc.borrow();
            let mut parts = Vec::new();
            for (k, val) in b.fields.iter() { parts.push(format!("{}: {}", k, fmt_value(val, depth-1))); }
            format!("{}{{{}}}", b.class, parts.join(", "))
        }
    }
}

