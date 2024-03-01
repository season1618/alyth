use std::fmt::{Display, Formatter, Result};

use Token::*;
use KeywordKind::*;
use PucntKind::*;

#[derive(Debug, Copy, Clone)]
pub enum Token<'a> {
    Punct(PucntKind),
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
pub enum PucntKind {
    Plus,
    Minus,
}

#[derive(Debug)]
pub enum Expr {
    Num(u32),
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

impl Display for PucntKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Plus => write!(f, "+"),
            Minus => write!(f, "-"),
        }
    }
}
