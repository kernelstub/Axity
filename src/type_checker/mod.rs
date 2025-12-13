use std::collections::HashMap;
use crate::ast::*;
use crate::error::{AxityError, Span};
use crate::types::Type;

pub fn check(p: &Program) -> Result<(), AxityError> {
    let mut funcs: HashMap<String, (Vec<Type>, Type, Span)> = HashMap::new();
    for it in &p.items {
        if let Item::Func(f) = it {
            if funcs.contains_key(&f.name) { return Err(AxityError::ty("duplicate function", f.span.clone())); }
            funcs.insert(f.name.clone(), (f.params.iter().map(|x| x.ty.clone()).collect(), f.ret.clone(), f.span.clone()));
        }
    }
    let mut vars: Vec<HashMap<String, Type>> = vec![HashMap::new()];
    for it in &p.items {
        match it {
            Item::Stmt(s) => check_stmt(s, &mut vars, &funcs)?,
            Item::Func(f) => {
                vars.push(HashMap::new());
                for par in &f.params { vars.last_mut().unwrap().insert(par.name.clone(), par.ty.clone()); }
                let mut has_return = false;
                for s in &f.body { if let Stmt::Return{..} = s { has_return = true; } check_stmt(s, &mut vars, &funcs)?; }
                vars.pop();
                if f.ret == Type::Int && !has_return { return Err(AxityError::ty("missing return", f.span.clone())); }
            }
        }
    }
    Ok(())
}

fn check_stmt(s: &Stmt, vars: &mut Vec<HashMap<String, Type>>, funcs: &HashMap<String,(Vec<Type>,Type,Span)>) -> Result<(), AxityError> {
    match s {
        Stmt::Let{ name, ty, init, span } => {
            let t = check_expr(init, vars, funcs)?;
            if &t != ty { return Err(AxityError::ty("type mismatch", span.clone())); }
            if vars.last().unwrap().contains_key(name) { return Err(AxityError::ty("duplicate variable", span.clone())); }
            vars.last_mut().unwrap().insert(name.clone(), ty.clone());
            Ok(())
        }
        Stmt::Assign{ name, expr, span } => {
            let t = check_expr(expr, vars, funcs)?;
            let vt = lookup_var(name, vars).ok_or_else(|| AxityError::ty("undefined variable", span.clone()))?;
            if vt != t { return Err(AxityError::ty("type mismatch", span.clone())); }
            Ok(())
        }
        Stmt::Print{ expr, .. } => { let _ = check_expr(expr, vars, funcs)?; Ok(()) }
        Stmt::While{ cond, body, span } => {
            let _ = check_expr(cond, vars, funcs)?;
            vars.push(HashMap::new());
            for st in body { check_stmt(st, vars, funcs)?; }
            vars.pop();
            Ok(())
        }
        Stmt::Return{ expr, .. } => { let _ = check_expr(expr, vars, funcs)?; Ok(()) }
    }
}

fn check_expr(e: &Expr, vars: &Vec<HashMap<String, Type>>, funcs: &HashMap<String,(Vec<Type>,Type,Span)>) -> Result<Type, AxityError> {
    match e {
        Expr::Int(_, _) => Ok(Type::Int),
        Expr::Var(name, sp) => lookup_var(name, vars).ok_or_else(|| AxityError::ty("undefined variable", sp.clone())),
        Expr::Binary{ left, right, .. } => { let lt = check_expr(left, vars, funcs)?; let rt = check_expr(right, vars, funcs)?; if lt==Type::Int && rt==Type::Int { Ok(Type::Int) } else { Err(AxityError::ty("binary type mismatch", span_of_expr(e))) } }
        Expr::Call{ name, args, span } => {
            let sig = funcs.get(name).ok_or_else(|| AxityError::ty("undefined function", span.clone()))?;
            if args.len() != sig.0.len() { return Err(AxityError::ty("argument count mismatch", span.clone())); }
            for (a,t) in args.iter().zip(sig.0.iter()) { let at = check_expr(a, vars, funcs)?; if &at != t { return Err(AxityError::ty("argument type mismatch", span.clone())); } }
            Ok(sig.1.clone())
        }
    }
}

fn lookup_var(name: &str, vars: &Vec<HashMap<String, Type>>) -> Option<Type> {
    for i in (0..vars.len()).rev() { if let Some(t) = vars[i].get(name) { return Some(t.clone()); } }
    None
}

fn span_of_expr(e: &Expr) -> Span {
    match e { Expr::Int(_, s) => s.clone(), Expr::Var(_, s) => s.clone(), Expr::Binary{ span, .. } => span.clone(), Expr::Call{ span, .. } => span.clone() }
}

