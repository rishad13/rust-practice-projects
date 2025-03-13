mod utils;

use crate::utils::{database::Database, index::Index};
use std::io::{self, BufRead, Write};
use std::process;

fn main() -> io::Result<()> {
    // Initialize databases for collections and index
    let mut collection_db = match Database::new("collections") {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to initialize collections database: {}", e);
            process::exit(1);
        }
    };

    let mut index_db = match Database::new("index") {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Failed to initialize index database: {}", e);
            process::exit(1);
        }
    };

    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut handle = stdin.lock();

    loop {
        // Display menu
        print_menu();
        stdout.flush()?;

        // Read command
        let mut input = String::new();
        handle.read_line(&mut input)?;

        match input.trim() {
            "ra" => read_all(&mut collection_db, &mut index_db)?,
            "rs" => read_single(&mut collection_db, &mut index_db, &mut handle)?,
            "cn" => create_new(&mut collection_db, &mut index_db, &mut handle)?,
            "us" => update_single(&mut collection_db, &mut index_db, &mut handle)?,
            "ds" => delete_single(&mut collection_db, &mut index_db, &mut handle)?,
            "exit" | "quit" => {
                println!("Exiting program");
                break;
            },
            "" => continue,
            _ => println!("Unknown command. Please try again."),
        }

        println!(); // Add blank line for better readability
    }

    Ok(())
}

fn print_menu() {
    println!("\nAvailable commands:");
    println!("ra - Read all entries");
    println!("ca - Write new entry");
    println!("rs - Read a single entry");
    println!("us - Update a single entry");
    println!("ds - Delete a single entry");
    println!("exit/quit - Exit the program");
    print!("> ");
}

fn read_all(collection_db: &mut Database, index_db: &mut Database) -> io::Result<()> {
    let indexes = match Index::get_all_indexing(index_db) {
        Ok(indexes) => indexes,
        Err(e) => {
            eprintln!("Failed to read index: {}", e);
            return Ok(());
        }
    };

    let mut found = false;
    for i in indexes {
        if i.is_free == 1 {
            continue;
        }
        found = true;
        match i.get_content(collection_db) {
            Ok(content) => println!("{} - {}", i.id, content),
            Err(e) => println!("{} - Error retrieving content: {}", i.id, e),
        }
    }

    if !found {
        println!("No entries found.");
    }

    Ok(())
}

fn read_single(collection_db: &mut Database, index_db: &mut Database, handle: &mut impl BufRead) -> io::Result<()> {
    let id = read_id(handle)?;

    let indexes = match Index::get_all_indexing(index_db) {
        Ok(indexes) => indexes,
        Err(e) => {
            eprintln!("Failed to read index: {}", e);
            return Ok(());
        }
    };

    match indexes.iter().find(|&x| x.id == id) {
        Some(index) => {
            match index.get_content(collection_db) {
                Ok(content) => println!("{}", content),
                Err(e) => println!("Error retrieving content: {}", e),
            }
        },
        None => println!("Entry with ID {} not found", id),
    }

    Ok(())
}

fn update_single(collection_db: &mut Database, index_db: &mut Database, handle: &mut impl BufRead) -> io::Result<()> {
    let id = read_id(handle)?;

    let indexes = match Index::get_all_indexing(index_db) {
        Ok(indexes) => indexes,
        Err(e) => {
            eprintln!("Failed to read index: {}", e);
            return Ok(());
        }
    };

    match indexes.iter().find(|&x| x.id == id) {
        Some(index) => {
            println!("Please provide the content to be updated:");
            print!("> ");
            io::stdout().flush()?;

            let mut content = String::new();
            handle.read_line(&mut content)?;

            match index.clone().update_with(collection_db, index_db, content) {
                Ok(updated_index) => {
                    match updated_index.get_content(collection_db) {
                        Ok(content) => println!("Updated successfully. New content: {}", content),
                        Err(e) => println!("Error retrieving updated content: {}", e),
                    }
                },
                Err(e) => println!("Failed to update: {}", e),
            }
        },
        None => println!("Entry with ID {} not found", id),
    }
    Ok(())
}

fn create_new(collection_db: &mut Database, index_db: &mut Database, handle: &mut impl BufRead) -> io::Result<()> { println!("Please provide the content to be updated:");
    print!("> ");
    io::stdout().flush()?;

    let mut content = String::new();
    handle.read_line(&mut content)?;
    let content = content.trim().to_string();
    Index::new(content, collection_db, index_db)?;
    Ok(())
}

fn delete_single(collection_db: &mut Database, index_db: &mut Database, handle: &mut impl BufRead) -> io::Result<()> {
    let id = read_id(handle)?;

    let indexes = match Index::get_all_indexing(index_db) {
        Ok(indexes) => indexes,
        Err(e) => {
            eprintln!("Failed to read index: {}", e);
            return Ok(());
        }
    };

    match indexes.iter().find(|&x| x.id == id) {
        //TODO: implement delete.
        Some(index) => {
            // Assuming there's a delete method in your Index struct
            // match index.clone().delete(collection_db, index_db) {
            //     Ok(_) => println!("Entry deleted successfully"),
            //     Err(e) => println!("Failed to delete: {}", e),
            // }
        },
        None => println!("Entry with ID {} not found", id),
    }

    Ok(())
}

fn read_id(handle: &mut impl BufRead) -> io::Result<u32> {
    println!("Please provide the ID:");
    print!("> ");
    io::stdout().flush()?;

    let mut string_id = String::new();
    handle.read_line(&mut string_id)?;

    match string_id.trim().parse::<u32>() {
        Ok(id) => Ok(id),
        Err(_) => {
            println!("Invalid ID format. Please enter a number.");
            read_id(handle)
        }
    }
}