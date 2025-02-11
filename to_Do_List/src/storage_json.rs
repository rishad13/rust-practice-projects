use crate::model::task_model;
use crate::model::task_model::Task;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

/// This function is used to store the data from the task_list in a
/// .json file. The .json file is created in the same directory as the
/// program and is named "storage.json". The function takes a vector of
/// the task_model type and uses serde to serialize the vector into a
/// string. Then the string is written to the file. The function does not
/// return anything and will panic if there is an error when creating or
/// writing to the file.
#[allow(dead_code)]
pub fn create_storage_store_data(task_list: &Vec<task_model::Task>) {
    let mut _file = File::create("storage.json").unwrap();
    let serialized = serde_json::to_string(&task_list).unwrap();
    _file.write_all(serialized.as_bytes()).unwrap();
}

/// Retrieves task data from "storage.json".
///
/// This function attempts to open and read the contents of a file named
/// "storage.json", which is expected to contain a JSON serialized vector
/// of `Task` objects. If the file is successfully read, the contents are
/// deserialized into a `Vec<Task>`. If the file does not exist, it is
/// created and an empty vector is returned. If any errors occur during
/// reading or deserialization, an error message is printed and an empty
/// vector is returned.
///
/// # Returns
///
/// A vector of `Task` objects retrieved from "storage.json", or an empty
/// vector if the file does not exist or cannot be read/parsed.

pub fn get_storage_data() -> Vec<Task> {
    let mut contents = String::new();

    let file = File::open("storage.json");

    match file {
        Ok(mut f) => {
            if let Err(err) = f.read_to_string(&mut contents) {
                eprintln!("Failed to read from storage.json: {}", err);
                return Vec::new();
            }
        }
        Err(_) => {
            // Create the file if it doesn't exist
            if let Err(err) = OpenOptions::new()
                .write(true)
                .create(true)
                .open("storage.json")
            {
                eprintln!("Failed to create storage.json: {}", err);
            }
            return Vec::new();
        }
    }

    if contents.trim().is_empty() {
        return Vec::new();
    }

    match serde_json::from_str::<Vec<Task>>(&contents) {
        Ok(tasks) => tasks,
        Err(err) => {
            eprintln!("Failed to parse JSON from storage.json: {}", err);
            Vec::new()
        }
    }
}
