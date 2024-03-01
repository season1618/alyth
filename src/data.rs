use std::fmt::{Display, Formatter, Result};

use Token::*;
use KeywordKind::*;
use PunctKind::*;

#[derive(Debug, Copy, Clone)]
pub enum Token<'a> {
    Punct(PunctKind),
    Keyword(KeywordKind),
    Ident(&'a str),
    Num(u32),
    Char(char),
    String(&'a str),
}

#[derive(Debug, Copy, Clone)]
pub enum KeywordKind {
    Let,
    Func,
}

#[derive(Debug, Copy, Clone)]
pub enum PunctKind {
    Plus,
    Minus,
}

#[derive(Debug)]
pub enum Expr {
    BinOp { kind: BinOpKind, lhs: Box<Expr>, rhs: Box<Expr> },
    Num(u32),
}

#[derive(Debug)]
pub enum BinOpKind {
    Add,
    Sub,
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Punct(punct) => punct.fmt(f),
            Keyword(keyword) => keyword.fmt(f),
            Ident(ident) => ident.fmt(f),
            Num(num) => num.fmt(f),
            Char(c) => c.fmt(f),
            String(s) => s.fmt(f),
        }
    }
}

impl Display for KeywordKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Let => write!(f, "let"),
            Func => write!(f, "func"),
        }
    }
}

impl Display for PunctKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
        }
    }
}
