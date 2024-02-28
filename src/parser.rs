use crate::data::*;

use Token::*;
use PucntKind::*;
use KeywordKind::*;
use Expr::*;

pub fn parse<'a>(tokens: Vec<Token<'a>>) -> Expr {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(tokens: Vec<Token<'a>>) -> Self {
        Parser {
            tokens,
            pos: 0,
        }
    }

    fn parse(&mut self) -> Expr {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Expr {
        match self.next() {
            Some(Token::Num(val)) => Expr::Num(val),
            _ => { panic!(); },
        }
    }

    fn next(&mut self) -> Option<Token<'a>> {
        if self.pos < self.tokens.len() {
            let token = self.tokens[self.pos];
            self.pos += 1;
            return Some(token);
        }
        None
    }
}