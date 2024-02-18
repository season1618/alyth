#[derive(Debug)]
pub enum Token<'a> {
    Num(u32),
    StrLit(&'a str),
}