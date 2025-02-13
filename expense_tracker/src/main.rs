use std::io;
use super:expense;

fn main() {
    loop {
        println!("1:Enter expense (amount category description):");
        println!("2: View all expenses");
        println!("3: Exit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim() {
            "1" => addExpense(),
            "2" => println!("View all expenses"),
            "3" => {
                println!("Bye Bye");
                break;
            }
            _ => println!("Invalid input"),
        }
    }
}
