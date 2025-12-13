use crate::error::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Let,
    Fn,
    Return,
    Print,
    While,
    IntType,
    Ident(String),
    IntLit(i64),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Colon,
    Semicolon,
    Comma,
    Arrow,
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    Less,
    LessEq,
    Greater,
    GreaterEq,
    EqEq,
    NotEq,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

