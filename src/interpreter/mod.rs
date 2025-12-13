use crate::ast::*;
use crate::error::AxityError;
use crate::runtime::{Runtime, Value};

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
        Stmt::Print{ expr, .. } => { let v = eval_expr(p, expr, rt, out)?; match v { Value::Int(i) => { out.push_str(&format!("{}\n", i)); } }; Ok(Control::Next) }
        Stmt::While{ cond, body, .. } => {
            loop {
                let c = eval_expr(p, cond, rt, out)?;
                let ci = match c { Value::Int(i) => i };
                if ci == 0 { break; }
                rt.push_scope();
                for st in body { if let Control::Return(_) = exec_stmt(p, st, rt, out)? { rt.pop_scope(); return Ok(Control::Next); } }
                rt.pop_scope();
            }
            Ok(Control::Next)
        }
        Stmt::Return{ expr, .. } => { let v = eval_expr(p, expr, rt, out)?; Ok(Control::Return(v)) }
    }
}

fn eval_expr(p: &Program, e: &Expr, rt: &mut Runtime, out: &mut String) -> Result<Value, AxityError> {
    match e {
        Expr::Int(i, _) => Ok(Value::Int(*i)),
        Expr::Var(name, _) => rt.get(name).ok_or_else(|| AxityError::rt("read of undefined variable")),
        Expr::Binary{ op, left, right, .. } => {
            let l = eval_expr(p, left, rt, out)?; let r = eval_expr(p, right, rt, out)?;
            let li = match l { Value::Int(i)=>i }; let ri = match r { Value::Int(i)=>i };
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
            };
            Ok(Value::Int(v))
        }
        Expr::Call{ name, args, .. } => {
            let mut ev_args = Vec::new();
            for a in args { ev_args.push(eval_expr(p, a, rt, out)?); }
            call_func(name, &ev_args, p, rt, out)
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

enum Control { Next, Return(Value) }

