pub mod expense;
use expense::expense_fn::{add_expense, list_all_expense};
use std::{collections::HashMap, io};

fn main() {
    let mut expense_list: Vec<HashMap<String, String>> = Vec::new();
    loop {
        println!("1:Enter expense (amount category description):");
        println!("2: View all expenses");
        println!("3: Exit");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim() {
            "1" => add_expense(&mut expense_list),
            "2" => list_all_expense(&mut expense_list),
            "3" => {
                println!("Bye Bye");
                break;
            }
            _ => println!("Invalid input"),
        }
    }
}
