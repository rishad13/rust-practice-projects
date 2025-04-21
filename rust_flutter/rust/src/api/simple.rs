use serde::{Deserialize, Serialize};

#[flutter_rust_bridge::frb(sync)] // Synchronous mode for simplicity of the demo
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

#[flutter_rust_bridge::frb(sync)]
pub fn get_demo_person() -> String {
    let person = Person {
        name: "John Doe".to_string(),
        age: 30,
        phones: vec!["123-456-7890".to_string(), "987-654-3210".to_string()],
    };
    serde_json::to_string(&person).unwrap()
}
