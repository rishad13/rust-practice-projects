use std::{collections::HashMap, io};
/// Splits the input string by the "-" delimiter and trims any leading or trailing whitespace.
///
/// # Arguments
///
/// * `input` - A string slice that holds the input to be split.
///
/// # Returns
///
/// * `Vec<&str>` - A vector of string slices obtained by splitting the input string.
pub fn add_expense(expense_list: &mut Vec<HashMap<String, String>>) {
    let mut input: String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let parts: Vec<&str> = input.trim().split("-").collect();
    if parts.len() == 3 {
        let mut expense = HashMap::new();
        expense.insert("amount".to_string(), parts[0].to_string());
        expense.insert("category".to_string(), parts[1].to_string());
        expense.insert("description".to_string(), parts[2].to_string());
        println!("added: {:?}", expense);
        expense_list.push(expense);
    } else {
        println!("Invalid input format. Please use the format: ₹amount - category - description");
    }
}

pub fn list_all_expense(expense_list: &mut Vec<HashMap<String, String>>) {
    for expense in expense_list {
        println!("{:?}", expense)
    }
}
