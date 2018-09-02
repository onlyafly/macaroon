extern crate quivi;

fn main() {
    let output = quivi::interpret("command-line", "1");
    println!("Hello: {}", output);
}
