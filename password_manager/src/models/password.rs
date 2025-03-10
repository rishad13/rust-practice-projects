use core::str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Password {
    pub name: String,
    pub password: PasswordCipher,
    pub id: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PasswordCipher {
    pub nonce: String,
    pub key: String,
    pub cipher_text: String,
}
