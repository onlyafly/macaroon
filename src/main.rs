mod interpreter;
mod scanner;

fn main() {
    println!("Hello: {}", interpreter::interpret("1"));
}
