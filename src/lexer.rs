use crate::data::*;

use Token::*;
use KeywordKind::*;

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

            if c.is_ascii_alphabetic() {
                tokens.push(self.read_keyword_ident());
                continue;
            }

            if c.is_ascii_digit() {
                tokens.push(self.read_num());
                continue;
            }

            tokens.push(StrLit(self.chs));
            break;
        }
        tokens
    }

    fn read_keyword_ident(&mut self) -> Token<'a> {
        let mut chs = self.chs.char_indices();
        let string = loop {
            match chs.clone().peekable().peek() {
                Some((_, c)) if c.is_ascii_alphanumeric() => {
                    chs.next();
                },
                Some((i, _)) => {
                    break &self.chs[..*i]
                },
                _ => {
                    break self.chs
                },
            }
        };
        self.chs = chs.as_str();

        match string {
            "let" => Keyword(Let),
            "func" => Keyword(Func),
            _ => Ident(string),
        }
    }

    fn read_num(&mut self) -> Token<'a> {
        let mut num = 0;
        let mut chs = self.chs.chars();
        loop {
            match chs.clone().peekable().peek() {
                Some(c) if c.is_ascii_digit() => {
                    num = 10 * num + c.to_digit(10).unwrap();
                    chs.next();
                },
                _ => {
                    self.chs = chs.as_str();
                    break;
                },
            }
        }
        Num(num)
    }

    fn peek_char(&self) -> Option<char> {
        self.chs.chars().next()
    }

    fn next_char(&mut self) -> Option<char> {
        let mut chs = self.chs.chars();
        match chs.next() {
            Some(c) => {
                self.chs = chs.as_str();
                Some(c)
            },
            None => None
        }
    }
}