use crate::data::*;

use Token::*;

pub fn tokenize<'a>(code: &'a str) -> Vec<Token<'a>> {
    let mut lexer = Lexer::new(code);
    lexer.tokenize()
}

struct Lexer<'a> {
    chs: &'a str,
}

impl<'a> Lexer<'a> {
    fn new(code: &'a str) -> Self {
        Lexer { chs: code }
    }

    fn tokenize(&mut self) -> Vec<Token<'a>> {
        vec![StrLit(self.chs)]
    }
}