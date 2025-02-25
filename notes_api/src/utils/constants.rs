use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref address: String = set_address();
    pub static ref port: u16 = set_port();
}

/// Loads the environment variables and retrieves the value of the "ADDRESS" variable.
/// If the "ADDRESS" variable is not set, it defaults to "127.0.0.1".
///
/// # Returns
///
/// A `String` containing the value of the "ADDRESS" environment variable or the default value.
fn set_address() -> String {
    env::var("ADDRESS").unwrap_or("127.0.0.1".to_string())
}

/// Loads the environment variables and retrieves the value of the "PORT" variable.
/// If the "PORT" variable is not set, it defaults to 8080.
///
/// # Panics
///
/// Panics if the "PORT" variable is not a number.
///
/// # Returns
///
/// A `u16` containing the value of the "PORT" environment variable or the default value.
fn set_port() -> u16 {
    env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .expect("PORT must be a number")
}
