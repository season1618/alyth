use std::io;
use std::fmt;

use TokenError::*;

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
