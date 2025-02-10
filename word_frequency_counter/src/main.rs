use std::collections::HashMap;
use std::io;

/// Reads a line of input from the user and prints a word frequency count.
///
/// This program prompts the user to enter a block of text, splits it into
/// individual words, and then prints a word frequency count. The output is
/// alphabetized by word.
///
fn main() {
    let mut input = String::new();
    println!("Enter a block of text:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let mut word_count = HashMap::new();

    for word in input.split_whitespace() {
        let count = word_count.entry(word.to_lowercase()).or_insert(0);
        *count += 1;
    }
    println!("\nWord Frequency Count:");
    for (input, count) in &word_count {
        println!("{}: {}", input, count);
    }
}
