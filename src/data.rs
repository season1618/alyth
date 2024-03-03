use std::fmt::{Display, Formatter, Result};

use Token::*;
use KeywordKind::*;
use PunctKind::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token<'a> {
    Punct(PunctKind),
    Keyword(KeywordKind),
    Ident(&'a str),
    Num(u32),
    Char(char),
    String(&'a str),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum KeywordKind {
    Let,
    Func,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PunctKind {
    OpenParen,
    CloseParen,
    SemiColon,
    VertVert,
    AndAnd,
    Ex,
    EqEq,
    ExEq,
    LtEq,
    Lt,
    GtEq,
    Gt,
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,
}

#[derive(Debug)]
pub enum Stmt {
    ExprStmt(Expr),
}

#[derive(Debug)]
pub enum Expr {
    BinOp { kind: BinOpKind, lhs: Box<Expr>, rhs: Box<Expr> },
    UnOp { kind: UnOpKind, operand: Box<Expr> },
    Num(u32),
}

#[derive(Debug)]
pub enum BinOpKind {
    LogicOr,
    LogicAnd,
    Eq,
    Neq,
    Leq,
    Lt,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Debug)]
pub enum UnOpKind {
    LogicNot,
    Neg,
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
            Let => "let".fmt(f),
            Func => "func".fmt(f),
        }
    }
}

impl Display for PunctKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            OpenParen => "(".fmt(f),
            CloseParen => ")".fmt(f),
            SemiColon => ";".fmt(f),
            VertVert => "||".fmt(f),
            AndAnd => "&&".fmt(f),
            Ex => "!".fmt(f),
            EqEq => "==".fmt(f),
            ExEq => "!=".fmt(f),
            LtEq => "<=".fmt(f),
            Lt => "<".fmt(f),
            GtEq => ">=".fmt(f),
            Gt => ">".fmt(f),
            Plus => "+".fmt(f),
            Minus => "-".fmt(f),
            Asterisk => "*".fmt(f),
            Slash => "/".fmt(f),
            Percent => "%".fmt(f),
        }
    }
}
