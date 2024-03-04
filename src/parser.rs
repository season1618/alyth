use crate::data::*;
use crate::error::ParseError;

use Token::*;
use PunctKind::*;
use KeywordKind::*;

use Defn::*;
use Stmt::*;
use Expr::*;
use BinOpKind::*;
use UnOpKind::*;
use ParseError::*;

pub fn parse<'a>(tokens: Vec<Token<'a>>) -> Result<Defn<'a>, ParseError<'a>> {
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

    fn parse(&mut self) -> Result<Defn<'a>, ParseError<'a>> {
        self.parse_defn()
    }

    fn parse_defn(&mut self) -> Result<Defn<'a>, ParseError<'a>> {
        match self.peek().ok_or(NoToken)? {
            Keyword(Func) => {
                self.next();
                let name = self.next_ident()?;
                self.consume(Punct(OpenParen))?;
                self.consume(Punct(CloseParen))?;

                let mut stmts = Vec::new();
                self.consume(Punct(OpenBrace))?;
                while Some(Punct(CloseBrace)) != self.peek() {
                    stmts.push(self.parse_stmt()?);
                }
                self.consume(Punct(CloseBrace))?;

                Ok(FuncDef { name, stmts })
            },
            actual => Err(UnexpectedToken { actual }),
        }
    }

    fn parse_stmt(&mut self) -> Result<Stmt, ParseError<'a>> {
        let stmt = match self.peek().ok_or(NoToken)? {
            Keyword(KeywordKind::Return) => {
                self.next();
                let expr = self.parse_expr()?;
                self.consume(Punct(SemiColon))?;
                Stmt::Return(expr)
            },
            _ => {
                let expr = self.parse_expr()?;
                self.consume(Punct(SemiColon))?;
                ExprStmt(expr)
            },
        };
        Ok(stmt)
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError<'a>> {
        self.parse_logic_or()
    }

    fn parse_logic_or(&mut self) -> Result<Expr, ParseError<'a>> {
        let mut lhs = self.parse_logic_and()?;
        while Some(Punct(VertVert)) == self.peek() {
            self.next();
            let rhs = self.parse_logic_and()?;
            lhs = BinOp { kind: LogicOr, lhs: Box::new(lhs), rhs: Box::new(rhs) };
        }
        Ok(lhs)
    }

    fn parse_logic_and(&mut self) -> Result<Expr, ParseError<'a>> {
        let mut lhs = self.parse_compare()?;
        while Some(Punct(AndAnd)) == self.peek() {
            self.next();
            let rhs = self.parse_compare()?;
            lhs = BinOp { kind: LogicAnd, lhs: Box::new(lhs), rhs: Box::new(rhs) };
        }
        Ok(lhs)
    }

    fn parse_compare(&mut self) -> Result<Expr, ParseError<'a>> {
        let mut lhs = self.parse_add()?;
        if let Some(token) = self.peek() {
            match token {
                Punct(EqEq) => {
                    self.next();
                    let rhs = self.parse_add()?;
                    lhs = BinOp { kind: Eq, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                },
                Punct(ExEq) => {
                    self.next();
                    let rhs = self.parse_add()?;
                    lhs = BinOp { kind: Neq, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                },
                Punct(LtEq) => {
                    self.next();
                    let rhs = self.parse_add()?;
                    lhs = BinOp { kind: Leq, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                },
                Punct(PunctKind::Lt) => {
                    self.next();
                    let rhs = self.parse_add()?;
                    lhs = BinOp { kind: BinOpKind::Lt, lhs: Box::new(lhs), rhs: Box::new(rhs) };
                },
                Punct(GtEq) => {
                    self.next();
                    let rhs = self.parse_add()?;
                    lhs = BinOp { kind: Leq, lhs: Box::new(rhs), rhs: Box::new(lhs) };
                },
                Punct(Gt) => {
                    self.next();
                    let rhs = self.parse_add()?;
                    lhs = BinOp { kind: BinOpKind::Lt, lhs: Box::new(rhs), rhs: Box::new(lhs) };
                },
                _ => {},
            }
        }
        Ok(lhs)
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
        let node = match self.peek().ok_or(NoToken)? {
            Punct(Ex) => {
                self.next();
                UnOp { kind: LogicNot, operand: Box::new(self.parse_unary()?) }
            },
            Punct(Minus) => {
                self.next();
                UnOp { kind: Neg, operand: Box::new(self.parse_unary()?) }
            },
            _ => {
                self.parse_primary()?
            },
        };
        Ok(node)
    }

    fn parse_primary(&mut self) -> Result<Expr, ParseError<'a>> {
        match self.peek().ok_or(NoToken)? {
            Punct(OpenParen) => {
                self.next();
                let expr = self.parse_expr()?;
                self.consume(Punct(CloseParen))?;
                Ok(expr)
            },
            Token::Num(val) => {
                self.next();
                Ok(Expr::Num(val))
            },
            other => {
                Err(UnexpectedToken { actual: other })
            },
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

    fn next_ident(&mut self) -> Result<&'a str, ParseError<'a>> {
        match self.next().ok_or(NoToken)? {
            Ident(ident) => Ok(ident),
            _ => Err(DroppedIdent),
        }
    }

    fn consume(&mut self, expected: Token<'a>) -> Result<(), ParseError<'a>> {
        if let Some(actual) = self.next() {
            if expected == actual {
                return Ok(());
            } else {
                return Err(MismatchedToken { expected, actual });
            }
        }
        Err(DroppedToken { expected })
    }
}