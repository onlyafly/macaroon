#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Error,
    EndOfFile,
    String(String),
    LeftParen,
    RightParen,
    Symbol(String),
    Number(String),
    Caret,
    SingleQuote,
    Char(String),
}

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub struct Loc {
    pub filename: String,
    pub pos: i32,
    pub line: i32,
}
