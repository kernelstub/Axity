use crate::error::Span;
use crate::types::Type;

#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub enum Item {
    Func(Function),
    Class(ClassDef),
    Import(String, Span),
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
    MemberAssign { object: Expr, field: String, expr: Expr, span: Span },
    Print { expr: Expr, span: Span },
    Expr(Expr),
    While { cond: Expr, body: Vec<Stmt>, span: Span },
    If { cond: Expr, then_body: Vec<Stmt>, else_body: Vec<Stmt>, span: Span },
    Return { expr: Expr, span: Span },
    Match { expr: Expr, arms: Vec<MatchArm>, default: Option<Vec<Stmt>>, span: Span },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64, Span),
    Str(String, Span),
    Bool(bool, Span),
    ArrayLit(Vec<Expr>, Span),
    Var(String, Span),
    New(String, Vec<Expr>, Span),
    Member { object: Box<Expr>, field: String, span: Span },
    Index { array: Box<Expr>, index: Box<Expr>, span: Span },
    MethodCall { object: Box<Expr>, name: String, args: Vec<Expr>, span: Span },
    UnaryNot { expr: Box<Expr>, span: Span },
    Binary { op: BinOp, left: Box<Expr>, right: Box<Expr>, span: Span },
    Call { name: String, args: Vec<Expr>, span: Span },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
}

#[derive(Debug, Clone)]
pub enum Pattern {
    PInt(i64),
    PStr(String),
    PBool(bool),
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pat: Pattern,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct ClassDef {
    pub name: String,
    pub fields: Vec<Field>,
    pub methods: Vec<Function>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}

