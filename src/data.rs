#[derive(Debug)]
pub enum Token<'a> {
    Keyword(KeywordKind),
    Ident(&'a str),
    Num(u32),
    StrLit(&'a str),
}

#[derive(Debug)]
pub enum KeywordKind {
    Let,
    Func,
}