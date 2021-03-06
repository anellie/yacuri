use crate::{lexer::Token, smol_str::SmolStr};
use alloc::{boxed::Box, vec::Vec};

#[derive(Debug)]
pub struct Module {
    pub path: Vec<SmolStr>,
    pub functions: Vec<Function>,
    pub classes: Vec<Class>,
}

#[derive(Debug)]
pub struct Class {
    pub name: Token,
    pub members: Vec<Member>,
    pub methods: Vec<Function>,
    pub functions: Vec<Function>,
}

#[derive(Debug)]
pub struct Member {
    pub name: Token,
    pub ty: Type,
    pub mutable: bool,
}

#[derive(Debug)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Parameter>,
    pub ret_type: Option<Type>,
    pub body: Option<Expr>,
}

#[derive(Debug)]
pub struct Parameter {
    pub name: SmolStr,
    pub ty: Type,
}

#[derive(Debug)]
pub struct Type {
    pub name: Token,
}

#[derive(Debug)]
pub struct Expr {
    pub ty: Box<EExpr>, // TODO use a bump allocator ideally
    pub start: usize,
}

#[derive(Debug)]
pub enum EExpr {
    Literal(Literal),

    Identifier(Token),

    Variable {
        final_: bool,
        name: Token,
        value: Expr,
    },

    Block(Vec<Expr>),

    If {
        cond: Expr,
        then: Expr,
        els: Option<Expr>,
    },

    While {
        cond: Expr,
        body: Expr,
    },

    Binary {
        left: Expr,
        op: Token,
        right: Expr,
    },

    Unary {
        op: Token,
        right: Expr,
    },

    Call {
        callee: Expr,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(SmolStr),
}
