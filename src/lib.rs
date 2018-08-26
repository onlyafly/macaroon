mod interpreter;
mod scanner;

pub fn interpret(s: &str) -> String {
    let tokens = interpreter::scan(s);
    let nodes = interpreter::parse(tokens);
    let result = interpreter::eval(nodes);

    result
}
