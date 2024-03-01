use std::io;
use std::fmt;

use crate::data::Token;
use TokenError::*;
use ParseError::*;

pub enum TokenError<'a> {
    InvalidChar(char),
    InvalidPunct(&'a str),
    UnterminatedCharLiteral,
    UnterminatedStringLiteral,
}

impl<'a> fmt::Display for TokenError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InvalidChar(c) => write!(f, "invalid character '{}'", c),
            InvalidPunct(s) => write!(f, "invalid punctuation '{}'", s),
            UnterminatedCharLiteral => write!(f, "unterminated character literal"),
            UnterminatedStringLiteral => write!(f, "unterminated string literal"),
        }
    }
}

pub enum ParseError<'a> {
    UnexpectedToken(Token<'a>),
    NoToken,
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnexpectedToken(token) => write!(f, "unexpected token '{}'", token),
            NoToken => write!(f, "no token"),
        }
    }
}
