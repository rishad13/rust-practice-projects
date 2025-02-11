mod model;
use std::io;

fn main() {
    println!("Welcome to Task Manager");
    println!("1. Add Task");
    println!("2. List Tasks");
    println!("3. Exit");
    let mut tasks: Vec<model::task_model::Task> = Vec::new();
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        match input {
            "1" => {
                println!("add task")
            }
            "2" => {
                println!("View Tasks")
            }
            "3" => {
                println!("Exiting... Goodbye!");
                break;
            }
            _ => println!("Invalid choice, please try again."),
        }
    }
}
