use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use rand::{rngs::OsRng, RngCore};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncryptionError {
    #[error("Failed to encrypt data: {0}")]
    EncryptionFailed(String),
    #[error("Failed to decrypt data: {0}")]
    DecryptionFailed(String),
    #[error("Invalid key length: {0}")]
    InvalidKeyLength(String),
    #[error("Invalid nonce length: {0}")]
    InvalidNonceLength(String),
}

/// Encrypts a string using AES-GCM encryption
/// 
/// # Arguments
/// * `data` - The string to encrypt
/// * `key` - The encryption key (must be 32 bytes)
/// 
/// # Returns
/// A base64 encoded string containing the encrypted data and nonce
pub fn encrypt(data: &str, key: &[u8]) -> Result<String, EncryptionError> {
    if key.len() != 32 {
        return Err(EncryptionError::InvalidKeyLength(format!(
            "Expected 32 bytes, got {}",
            key.len()
        )));
    }

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, data.as_bytes())
        .map_err(|e| EncryptionError::EncryptionFailed(e.to_string()))?;

    // Combine nonce and ciphertext
    let mut combined = Vec::with_capacity(nonce_bytes.len() + ciphertext.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&ciphertext);

    Ok(BASE64.encode(&combined))
}

/// Decrypts a base64 encoded string that was encrypted using AES-GCM
/// 
/// # Arguments
/// * `encrypted_data` - The base64 encoded encrypted data
/// * `key` - The encryption key (must be 32 bytes)
/// 
/// # Returns
/// The decrypted string
pub fn decrypt(encrypted_data: &str, key: &[u8]) -> Result<String, EncryptionError> {
    if key.len() != 32 {
        return Err(EncryptionError::InvalidKeyLength(format!(
            "Expected 32 bytes, got {}",
            key.len()
        )));
    }

    let combined = BASE64
        .decode(encrypted_data)
        .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))?;

    if combined.len() < 12 {
        return Err(EncryptionError::InvalidNonceLength(format!(
            "Data too short to contain nonce, got {} bytes",
            combined.len()
        )));
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let nonce = Nonce::from_slice(nonce_bytes);
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))?;

    String::from_utf8(plaintext).map_err(|e| EncryptionError::DecryptionFailed(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encryption_decryption() {
        let key = [1u8; 32];
        let data = "test data";

        let encrypted = encrypt(data, &key).unwrap();
        let decrypted = decrypt(&encrypted, &key).unwrap();

        assert_eq!(data, decrypted);
    }

    #[test]
    fn test_invalid_key_length() {
        let key = [1u8; 16];
        let data = "test data";

        assert!(matches!(
            encrypt(data, &key),
            Err(EncryptionError::InvalidKeyLength(_))
        ));
        assert!(matches!(
            decrypt("test", &key),
            Err(EncryptionError::InvalidKeyLength(_))
        ));
    }
} 