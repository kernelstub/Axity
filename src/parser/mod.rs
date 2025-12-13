use crate::ast::*;
use crate::error::{AxityError, Span};
use crate::token::{Token, TokenKind};
use crate::types::Type;

pub fn parse(tokens: &[Token]) -> Result<Program, AxityError> {
    let mut p = Parser { tokens, i: 0 };
    p.program()
}

struct Parser<'a> {
    tokens: &'a [Token],
    i: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> &'a Token { &self.tokens[self.i] }
    fn next(&mut self) -> &'a Token { let t = &self.tokens[self.i]; self.i += 1; t }
    fn expect(&mut self, kind: TokenKind) -> Result<Token, AxityError> {
        let t = self.next().clone();
        if t.kind == kind { Ok(t) } else { Err(AxityError::parse("unexpected token", t.span)) }
    }
    fn program(&mut self) -> Result<Program, AxityError> {
        let mut items = Vec::new();
        while self.peek().kind != TokenKind::Eof {
            if self.peek().kind == TokenKind::Fn { items.push(Item::Func(self.function()?)); }
            else { items.push(Item::Stmt(self.statement()?)); }
        }
        Ok(Program { items })
    }
    fn function(&mut self) -> Result<Function, AxityError> {
        let fn_tok = self.expect(TokenKind::Fn)?;
        let name = match self.next().kind.clone() { TokenKind::Ident(s) => s, _ => return Err(AxityError::parse("expected identifier", fn_tok.span)) };
        self.expect(TokenKind::LParen)?;
        let mut params = Vec::new();
        if self.peek().kind != TokenKind::RParen {
            loop {
                let pname = match self.next().kind.clone() { TokenKind::Ident(s) => s, _ => return Err(AxityError::parse("expected identifier", self.peek().span.clone())) };
                self.expect(TokenKind::Colon)?;
                let ty = self.parse_type()?;
                params.push(Param{ name: pname, ty, span: fn_tok.span.clone() });
                if self.peek().kind == TokenKind::Comma { self.next(); } else { break; }
            }
        }
        self.expect(TokenKind::RParen)?;
        self.expect(TokenKind::Arrow)?;
        let ret = self.parse_type()?;
        self.expect(TokenKind::LBrace)?;
        let mut body = Vec::new();
        while self.peek().kind != TokenKind::RBrace { body.push(self.statement()?); }
        self.expect(TokenKind::RBrace)?;
        Ok(Function { name, params, ret, body, span: fn_tok.span })
    }
    fn parse_type(&mut self) -> Result<Type, AxityError> {
        let t = self.next().clone();
        match t.kind { TokenKind::IntType => Ok(Type::Int), _ => Err(AxityError::parse("unknown type", t.span)) }
    }
    fn statement(&mut self) -> Result<Stmt, AxityError> {
        match self.peek().kind.clone() {
            TokenKind::Let => {
                let lt = self.next().span.clone();
                let name = match self.next().kind.clone() { TokenKind::Ident(s) => s, _ => return Err(AxityError::parse("expected identifier", lt)) };
                self.expect(TokenKind::Colon)?;
                let ty = self.parse_type()?;
                self.expect(TokenKind::Assign)?;
                let init = self.expr()?;
                self.expect(TokenKind::Semicolon)?;
                Ok(Stmt::Let{ name, ty, init, span: lt })
            }
            TokenKind::Print => {
                let sp = self.next().span.clone();
                self.expect(TokenKind::LParen)?;
                let e = self.expr()?;
                self.expect(TokenKind::RParen)?;
                self.expect(TokenKind::Semicolon)?;
                Ok(Stmt::Print{ expr: e, span: sp })
            }
            TokenKind::While => {
                let sp = self.next().span.clone();
                let cond = self.expr()?;
                self.expect(TokenKind::LBrace)?;
                let mut body = Vec::new();
                while self.peek().kind != TokenKind::RBrace { body.push(self.statement()?); }
                self.expect(TokenKind::RBrace)?;
                Ok(Stmt::While{ cond, body, span: sp })
            }
            TokenKind::Return => {
                let sp = self.next().span.clone();
                let e = self.expr()?;
                self.expect(TokenKind::Semicolon)?;
                Ok(Stmt::Return{ expr: e, span: sp })
            }
            TokenKind::Ident(_) => {
                let name = if let TokenKind::Ident(s) = self.next().kind.clone() { s } else { unreachable!() };
                if self.peek().kind == TokenKind::Assign {
                    let sp = self.next().span.clone();
                    let e = self.expr()?;
                    self.expect(TokenKind::Semicolon)?;
                    Ok(Stmt::Assign{ name, expr: e, span: sp })
                } else if self.peek().kind == TokenKind::LParen {
                    self.expect(TokenKind::LParen)?;
                    let mut args = Vec::new();
                    if self.peek().kind != TokenKind::RParen {
                        loop {
                            args.push(self.expr()?);
                            if self.peek().kind == TokenKind::Comma { self.next(); } else { break; }
                        }
                    }
                    self.expect(TokenKind::RParen)?;
                    self.expect(TokenKind::Semicolon)?;
                    Ok(Stmt::Print{ expr: Expr::Call{ name, args, span: self.peek().span.clone() }, span: self.peek().span.clone() })
                } else {
                    Err(AxityError::parse("invalid statement", self.peek().span.clone()))
                }
            }
            _ => Err(AxityError::parse("unexpected token in statement", self.peek().span.clone()))
        }
    }
    fn expr(&mut self) -> Result<Expr, AxityError> { self.expr_eq() }
    fn expr_eq(&mut self) -> Result<Expr, AxityError> {
        let mut e = self.expr_rel()?;
        loop {
            let k = self.peek().kind.clone();
            if k == TokenKind::EqEq || k == TokenKind::NotEq {
                let op = if k == TokenKind::EqEq { BinOp::Eq } else { BinOp::Ne };
                let sp = self.next().span.clone();
                let r = self.expr_rel()?;
                e = Expr::Binary{ op, left: Box::new(e), right: Box::new(r), span: sp };
            } else { break; }
        }
        Ok(e)
    }
    fn expr_rel(&mut self) -> Result<Expr, AxityError> {
        let mut e = self.expr_add()?;
        loop {
            let k = self.peek().kind.clone();
            let op = match k { TokenKind::Less => Some(BinOp::Lt), TokenKind::LessEq => Some(BinOp::Le), TokenKind::Greater => Some(BinOp::Gt), TokenKind::GreaterEq => Some(BinOp::Ge), _ => None };
            if let Some(op) = op { let sp = self.next().span.clone(); let r = self.expr_add()?; e = Expr::Binary{ op, left: Box::new(e), right: Box::new(r), span: sp }; } else { break; }
        }
        Ok(e)
    }
    fn expr_add(&mut self) -> Result<Expr, AxityError> {
        let mut e = self.expr_mul()?;
        loop {
            let k = self.peek().kind.clone();
            let op = match k { TokenKind::Plus => Some(BinOp::Add), TokenKind::Minus => Some(BinOp::Sub), _ => None };
            if let Some(op) = op { let sp = self.next().span.clone(); let r = self.expr_mul()?; e = Expr::Binary{ op, left: Box::new(e), right: Box::new(r), span: sp }; } else { break; }
        }
        Ok(e)
    }
    fn expr_mul(&mut self) -> Result<Expr, AxityError> {
        let mut e = self.expr_primary()?;
        loop {
            let k = self.peek().kind.clone();
            let op = match k { TokenKind::Star => Some(BinOp::Mul), TokenKind::Slash => Some(BinOp::Div), _ => None };
            if let Some(op) = op { let sp = self.next().span.clone(); let r = self.expr_primary()?; e = Expr::Binary{ op, left: Box::new(e), right: Box::new(r), span: sp }; } else { break; }
        }
        Ok(e)
    }
    fn expr_primary(&mut self) -> Result<Expr, AxityError> {
        let t = self.next().clone();
        match t.kind {
            TokenKind::IntLit(v) => Ok(Expr::Int(v, t.span)),
            TokenKind::Ident(ref s) => {
                if self.peek().kind == TokenKind::LParen {
                    self.expect(TokenKind::LParen)?;
                    let mut args = Vec::new();
                    if self.peek().kind != TokenKind::RParen {
                        loop { args.push(self.expr()?); if self.peek().kind == TokenKind::Comma { self.next(); } else { break; } }
                    }
                    self.expect(TokenKind::RParen)?;
                    Ok(Expr::Call{ name: s.clone(), args, span: t.span })
                } else { Ok(Expr::Var(s.clone(), t.span)) }
            }
            TokenKind::LParen => { let e = self.expr()?; self.expect(TokenKind::RParen)?; Ok(e) }
            _ => Err(AxityError::parse("unexpected token in expression", t.span))
        }
    }
}

