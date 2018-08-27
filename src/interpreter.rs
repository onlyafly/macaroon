use scanner;
use tokens::Token;

pub fn scan(s: &str) -> Vec<Token> {
    let _scanner = scanner::Scanner::new(s);

    vec![Token::Number(s.to_string())]
}

pub fn parse(tokens: Vec<Token>) -> String {
    let t = &tokens[0];
    match t {
        Token::Number(s) => s.clone(),
        _ => "unknown".to_string(),
    }
}

pub fn eval(s: String) -> String {
    s
}
