use crate::{
    models::password::Password,
    utils::{
        self,
        storage::{create_storage_and_store_data, get_storage_data},
    },
};

/// A loop that prints a menu and calls other functions based on user input.
/// The menu is:
/// 1. add
/// 2. list
/// 3. delete
/// 4. exit
/// 5. get password
///
pub fn control_cli() {
    let mut password: Vec<Password> = get_storage_data();
    loop {
        println!("1. add\n2. list,\n3. delete,\n4. exit, \n5. get password");
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim() {
            "1" => add_password(&mut password),
            "2" => list_password(&mut password),
            "3" => delete_password(&mut password),
            "4" => {
                println!("bye");
                break;
            }
            "5" => get_password(&mut password),
            _ => println!("Invalid command"),
        }
    }
}

/// Retrieves a password from the list and decrypts it, given an id.
///
/// It lists all the passwords and then asks the user to input the id of the password he wants to
/// retrieve. If the id is valid, it will print the decrypted password.
fn get_password(password_list: &mut Vec<Password>) {
    list_password(password_list);
    let mut name = String::new();
    println!("Enter website ID:");
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let id = match name.trim().parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid id");
            return;
        }
    };
    for i in password_list {
        if i.id == id {
            println!("Password:{}", utils::encryption::decrypt(&i.password));
        }
    }
}

/// Adds a new password to the list by prompting the user for a website name and password.
/// The password is encrypted before being stored.
/// The updated password list is then saved to the storage.
/// The ID of the new password is set as the current length of the list.
fn add_password(password_list: &mut Vec<Password>) {
    let mut name = String::new();
    println!("Enter website:");
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    println!("Enter password:");
    let mut password = String::new();
    std::io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");
    let password_cipher = utils::encryption::encrypt(&password);
    let new_password = Password {
        name: name.to_string(),
        password: password_cipher,
        id: password_list.len(),
    };
    password_list.push(new_password);
    create_storage_and_store_data(password_list);
    println!("Password added");
}

/// Lists all stored passwords with their id and name.
fn list_password(password_list: &mut Vec<Password>) {
    for i in password_list {
        println!("Id:{}  Name:{}", i.id, i.name);
    }
}

/// Deletes a password from the list by asking the user for an id.
/// The password list is then saved to the storage.
///
fn delete_password(password_list: &mut Vec<Password>) {
    list_password(password_list);
    let mut name = String::new();
    println!("Enter website Id:");
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    let id = match name.trim().parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid id");
            return;
        }
    };
    password_list.retain(|p| p.id != id);
    create_storage_and_store_data(password_list);
    println!("Password deleted");
}
