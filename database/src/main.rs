use crate::utils::{database::Database, index::Index};
use io::stdin;
use std::io;
mod utils;
fn main() {
    // Initialize databases for collections and index
    let mut collection_db = Database::new("collections").unwrap();
    let mut index_db = Database::new("index").unwrap();

    // Clear any existing data in both databases
    // Index::clear(&mut collection_db, &mut index_db).unwrap();

    let mut input = String::new();
    println!("choose command (ra, rs, us, ds)");
    match stdin().read_line(&mut input) {
        Ok(_) => match input.trim() {
            "ra" => {
                let indexes = Index::get_all_indexing(&mut index_db).expect("Failed to read index");
                for i in indexes {
                    if i.is_free == 1 {
                        continue;
                    }
                    println!("{} - {}", i.id, i.get_content(&mut collection_db).unwrap())
                }
            }
            "rs" => {
                println!("Please provide the id");
                let mut string_id = String::new();
                stdin().read_line(&mut string_id).expect("failed to read");
                let mut indexes =
                    Index::get_all_indexing(&mut index_db).expect("Failed to read index");
                let i: u32 = string_id.trim().parse().expect("Please type a number!");
                let index = indexes.iter().find(|&x| x.id == i).expect("Not found");
                println!("{}", index.get_content(&mut collection_db).unwrap());
            }
            "us" => {
                println!("Please provide the id");
                let mut string_id = String::new();
                stdin().read_line(&mut string_id).expect("failed to read");
                let i: u32 = string_id.trim().parse().expect("Please type a number!");
                let mut indexes =
                    Index::get_all_indexing(&mut index_db).expect("Failed to read index");
                let index = indexes.iter().find(|&x| x.id == i).expect("Not found");
                println!("Please provide the content to be updated");
                let mut content = String::new();
                stdin().read_line(&mut content).expect("failed to read");
                let updated_index = index.clone().update_with(&mut collection_db, &mut index_db, content).expect("failed to update");
                println!("changed to {}", updated_index.get_content(&mut collection_db).unwrap());
            }
            "ds" => {}
            _ => {}
        },
        Err(_) => {}
    }
    // println!("{}", input.lines().collect());
}
