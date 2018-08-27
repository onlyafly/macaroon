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
