use crate::{model::task_model, storage_json::create_storage_store_data};
use std::io;

/// Adds a new task to the task list.
///
/// Prompts the user to enter a description and assigns an id of
/// `taskList.len() + 1` to the new task.
pub fn add_task(task_list: &mut Vec<task_model::Task>) {
    let mut input = String::new();
    println!("Enter task task name: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let task = task_model::Task {
        id: task_list.len() + 1,
        description: input,
    };

    task_list.push(task);
    println!("Task added successfully!");
    create_storage_store_data(task_list);
}

/// Lists all tasks in the task list.
///
/// Iterates over each task in the provided `taskList` vector and prints
/// the task id and description in the format "id: description".
///
/// # Arguments
///
/// * `taskList` - A reference to a vector of `Task` objects to be listed.
pub fn list_tasks(task_list: &Vec<task_model::Task>) {
    for task in task_list {
        println!("{}: {}", task.id, task.description);
    }
}
