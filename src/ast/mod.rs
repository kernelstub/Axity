use crate::error::Span;
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Func(Function),
    Stmt(Stmt),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub ret: Type,
    pub body: Vec<Stmt>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, ty: Type, init: Expr, span: Span },
    Assign { name: String, expr: Expr, span: Span },
    Print { expr: Expr, span: Span },
    While { cond: Expr, body: Vec<Stmt>, span: Span },
    Return { expr: Expr, span: Span },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64, Span),
    Var(String, Span),
    Binary { op: BinOp, left: Box<Expr>, right: Box<Expr>, span: Span },
    Call { name: String, args: Vec<Expr>, span: Span },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

