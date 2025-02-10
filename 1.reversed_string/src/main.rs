use std::io;
/// Reads a line of input from the user and prints it back out in reverse order.
///
fn main() {
    let mut input = String::new();
    println!("Enter a string:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().chars().rev().collect::<String>();
    println!("{}", input);
}
