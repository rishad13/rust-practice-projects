use crate::models::password::PasswordCipher;
use aes_gcm::{
    aead::{Aead, OsRng},
    AeadCore, Aes256Gcm, Key, KeyInit, Nonce,
};
use base64::{engine::general_purpose, Engine};

/// Encrypts the given plaintext using the AES-256-GCM encryption algorithm.
/// Generates a random key and nonce for each encryption process.
/// The resulting encrypted data, along with the key and nonce, is encoded in base64
/// and returned as a `PasswordCipher` object.
///
/// # Arguments
///
/// * `plaintext` - A string slice that holds the data to be encrypted.
///
/// # Returns
///
/// * `PasswordCipher` - A struct containing the base64-encoded key, nonce, and cipher text.
pub fn encrypt(plaintext: &str) -> PasswordCipher {
    let key = Aes256Gcm::generate_key(&mut OsRng);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let cipher = Aes256Gcm::new(&key);
    let cipher_text = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .expect("Failed to encrypt");
    let encoded_key = general_purpose::STANDARD.encode(key);
    let encoded_nonce = general_purpose::STANDARD.encode(nonce);
    let encoded_cipher_text = general_purpose::STANDARD.encode(cipher_text);
    return PasswordCipher {
        key: encoded_key,
        nonce: encoded_nonce,
        cipher_text: encoded_cipher_text,
    };
}

/// Decrypts the given `PasswordCipher` containing encrypted data.
/// Utilizes the AES-256-GCM encryption algorithm to decrypt the cipher text
/// with the provided base64-encoded key and nonce.
/// Returns the original plaintext as a `String`.
///
/// # Arguments
///
/// * `password_cipher` - A reference to a `PasswordCipher` struct containing
///   the base64-encoded key, nonce, and cipher text to be decrypted.
///
/// # Returns
///
/// * `String` - The decrypted plaintext as a UTF-8 encoded string.
pub fn decrypt(password_cipher: &PasswordCipher) -> String {
    let key_bytes = general_purpose::STANDARD
        .decode(&password_cipher.key)
        .expect("Failed to decode key");
    let cipher_bytes = general_purpose::STANDARD
        .decode(&password_cipher.cipher_text)
        .expect("Failed to decode cipher text");
    let nonce_bytes = general_purpose::STANDARD
        .decode(&password_cipher.nonce)
        .expect("Failed to decode nonce");
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&key_bytes));
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Decrypt the data
    let decrypted_bytes = cipher
        .decrypt(nonce, cipher_bytes.as_ref())
        .expect("Decryption failed");
    return String::from_utf8(decrypted_bytes).expect("Invalid UTF-8");
}
