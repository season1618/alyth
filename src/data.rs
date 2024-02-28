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