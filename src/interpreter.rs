use scanner;

enum Token {
    Nil,
    Generic(String),
}

pub fn interpret(s: &str) -> String {
    let tokens = scan(s);
    let nodes = parse(tokens);
    let result = eval(nodes);

    result
}

fn scan(s: &str) -> Vec<Token> {
    let scanner = scanner::Scanner::new(s);

    vec![Token::Generic(s.to_string())]
}

fn parse(tokens: Vec<Token>) -> String {
    let t = &tokens[0];
    match t {
        Token::Generic(s) => s.clone(),
        Token::Nil => "nil".to_string(),
    }
}

fn eval(s: String) -> String {
    s
}
