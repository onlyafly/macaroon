#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Error(String),
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

impl Token {
    pub fn display(&self) -> String {
        #[allow(unreachable_patterns)]
        match self {
            &Token::Error(ref s) => format!("{}", s),
            &Token::EndOfFile => "<eof>".to_string(),
            &Token::String(ref s) => format!("\"{}\"", s),
            &Token::LeftParen => "(".to_string(),
            &Token::RightParen => "(".to_string(),
            &Token::Symbol(ref s) => s.clone(),
            &Token::Number(ref s) => s.clone(),
            &Token::Caret => "^".to_string(),
            &Token::SingleQuote => "'".to_string(),
            &Token::Char(ref s) => format!("'{}", s),
        }
    }
}
