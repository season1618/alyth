#[derive(Debug)]
pub enum Token<'a> {
    StrLit(&'a str),
}