use scanner;
use tokens::*;

pub fn scan(s: &str) -> Vec<Token> {
    let _scanner = scanner::Scanner::new(s);

    vec![Token::Generic(s.to_string())]
}

pub fn parse(tokens: Vec<Token>) -> String {
    let t = &tokens[0];
    match t {
        Token::Generic(s) => s.clone(),
        Token::Nil => "nil".to_string(),
    }
}

pub fn eval(s: String) -> String {
    s
}
