#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Error(String),
    EndOfFile,
    //TODO: String(String),
    LeftParen,
    RightParen,
    Symbol(String),
    Number(String),
    StringLiteral(String),
    Caret,
    SingleQuote,
    Character { val: String, raw: String },
}

impl Token {
    pub fn display(&self) -> String {
        match self {
            &Token::Error(ref s) => format!("{}", s),
            &Token::EndOfFile => "<eof>".to_string(),
            //TODO: &Token::String(ref s) => format!("\"{}\"", s),
            &Token::LeftParen => "(".to_string(),
            &Token::RightParen => "(".to_string(),
            &Token::Symbol(ref s) => s.clone(),
            &Token::Number(ref s) => s.clone(),
            &Token::Caret => "^".to_string(),
            &Token::SingleQuote => "'".to_string(),
            &Token::Character { ref raw, .. } => format!("{}", raw),
            &Token::StringLiteral(ref s) => format!("\"{}\"", s),
        }
    }
}
