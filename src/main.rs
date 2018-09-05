extern crate quivi;

// TODO: see https://mgattozzi.com/scheme-parser for how to write a good input line
fn main() {
    let output = quivi::interpret("command-line", "1");
    println!("Hello: {}", output);
}
