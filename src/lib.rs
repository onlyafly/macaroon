// Assume that crate is called adder, will have to extern it in integration test.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

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
