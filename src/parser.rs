use crate::data::*;
use crate::error::ParseError;

use Token::*;
use PunctKind::*;
use KeywordKind::*;
use Expr::*;
use BinOpKind::*;
use UnOpKind::*;
use ParseError::*;

pub fn parse<'a>(tokens: Vec<Token<'a>>) -> Result<Expr, ParseError<'a>> {
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

    fn parse(&mut self) -> Result<Expr, ParseError<'a>> {
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError<'a>> {
        self.parse_add()
    }

    fn parse_add(&mut self) ->  Result<Expr, ParseError<'a>> {
        let mut lhs = self.parse_mul()?;
        while let Some(token) = self.peek() {
            match token {
                Punct(Plus) => {
                    self.next();
                    let rhs = self.parse_mul()?;
                    lhs = BinOp { kind: Add, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                },
                Punct(Minus) => {
                    self.next();
                    let rhs = self.parse_mul()?;
                    lhs = BinOp { kind: Sub, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                },
                _ => {
                    break;
                },
            }
        }
        Ok(lhs)
    }

    fn parse_mul(&mut self) ->  Result<Expr, ParseError<'a>> {
        let mut lhs = self.parse_unary()?;
        while let Some(token) = self.peek() {
            match token {
                Punct(Asterisk) => {
                    self.next();
                    let rhs = self.parse_unary()?;
                    lhs = BinOp { kind: Mul, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                },
                Punct(Slash) => {
                    self.next();
                    let rhs = self.parse_unary()?;
                    lhs = BinOp { kind: Div, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                },
                Punct(Percent) => {
                    self.next();
                    let rhs = self.parse_unary()?;
                    lhs = BinOp { kind: Mod, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                }
                _ => {
                    break;
                },
            }
        }
        Ok(lhs)
    }

    fn parse_unary(&mut self) -> Result<Expr, ParseError<'a>> {
        if let Some(token) = self.peek() {
            let node = match token {
                Punct(Minus) => {
                    self.next();
                    UnOp { kind: Neg, operand: Box::new(self.parse_unary()?) }
                },
                _ => {
                    self.parse_primary()?
                },
            };
            return Ok(node);
        }
        Err(NoToken)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError<'a>> {
        match self.next() {
            Some(Token::Num(val)) => Ok(Expr::Num(val)),
            Some(token) => Err(UnexpectedToken(token)),
            None => Err(NoToken),
        }
    }

    fn peek(&self) -> Option<Token<'a>> {
        if self.pos < self.tokens.len() {
            return Some(self.tokens[self.pos]);
        }
        None
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