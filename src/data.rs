#[derive(Debug)]
pub enum Token<'a> {
    Punct(PucntKind),
    Keyword(KeywordKind),
    Ident(&'a str),
    Num(u32),
    Char(char),
    String(&'a str),
}

#[derive(Debug)]
pub enum KeywordKind {
    Let,
    Func,
}

#[derive(Debug)]
pub enum PucntKind {
    Plus,
    Minus,
}