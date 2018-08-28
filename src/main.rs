extern crate quivi;

fn main() {
    let result_node = quivi::interpret("1");
    let result_display = result_node.display();

    println!("Hello: {}", result_display);
}
