use crate::models::password::{self};
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

/// Creates the storage file and stores the given password list in it.
/// Panics if the storage file can't be created or written to.
///
pub fn create_storage_and_store_data(password_list: &Vec<password::Password>) {
    let mut _file = File::create("storage.json").unwrap();
    let serialized = serde_json::to_string(&password_list).expect("Failed to serialize");
    _file
        .write_all(serialized.as_bytes())
        .expect("Failed to write to file");
}

/// Retrieves the stored passwords from the storage file.
///
/// The storage file is created if it doesn't exist.
/// If the file can't be read or the JSON can't be parsed,
/// an error is printed to stderr and an empty vec is returned.
pub fn get_storage_data() -> Vec<password::Password> {
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

    match serde_json::from_str::<Vec<password::Password>>(&contents) {
        Ok(tasks) => tasks,
        Err(err) => {
            eprintln!("Failed to parse JSON from storage.json: {}", err);
            Vec::new()
        }
    }
}
