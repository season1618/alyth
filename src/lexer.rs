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
        let mut tokens = Vec::new();
        while let Some(c) = self.peek_char() {
            if c.is_whitespace() {
                self.next_char();
                continue;
            }
            tokens.push(StrLit(self.chs));
            break;
        }
        tokens
    }

    fn peek_char(&self) -> Option<char> {
        if let Some(c) = self.chs.chars().nth(0) {
            return Some(c);
        }
        None
    }

    fn next_char(&mut self) -> Option<char> {
        if let Some(c) = self.chs.chars().nth(0) {
            let i = if let Some((i, _)) = self.chs.char_indices().nth(1) { i } else { self.chs.len() };
            self.chs = &self.chs[i..];
            return Some(c);
        }
        None
    }
}