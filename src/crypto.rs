use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce,
};
use anyhow::{Context, Result};
use std::sync::OnceLock;

static ENCRYPTION_KEY: OnceLock<[u8; 32]> = OnceLock::new();

/// Initialize the encryption key (should be called once at startup)
/// In production, this should be loaded from environment variable or secure key management
pub fn init_encryption_key(key: Option<&str>) -> Result<()> {
    let key_bytes = if let Some(k) = key {
        // Use provided key - pad or truncate to 32 bytes
        let mut key = [0u8; 32];
        let bytes = k.as_bytes();
        let len = bytes.len().min(32);
        key[..len].copy_from_slice(&bytes[..len]);
        key
    } else {
        // Generate a random key (for demo purposes)
        // In production, you'd want to persist this key securely
        let mut key = [0u8; 32];
        use aes_gcm::aead::rand_core::RngCore;
        OsRng.fill_bytes(&mut key);
        key
    };

    ENCRYPTION_KEY
        .set(key_bytes)
        .map_err(|_| anyhow::anyhow!("Encryption key already initialized"))?;

    Ok(())
}

/// Get the encryption key
fn get_key() -> Result<&'static [u8; 32]> {
    ENCRYPTION_KEY
        .get()
        .context("Encryption key not initialized. Call init_encryption_key first")
}

/// Encrypt a JSON string
pub fn encrypt_data(plaintext: &str) -> Result<Vec<u8>> {
    let key = get_key()?;
    let cipher = Aes256Gcm::new(key.into());

    // Generate a random nonce (12 bytes for AES-GCM)
    let mut nonce_bytes = [0u8; 12];
    use aes_gcm::aead::rand_core::RngCore;
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    // Encrypt the data
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;

    // Prepend nonce to ciphertext (we need it for decryption)
    let mut result = nonce_bytes.to_vec();
    result.extend_from_slice(&ciphertext);

    Ok(result)
}

/// Decrypt encrypted data
pub fn decrypt_data(encrypted: &[u8]) -> Result<String> {
    if encrypted.len() < 12 {
        anyhow::bail!("Invalid encrypted data: too short");
    }

    let key = get_key()?;
    let cipher = Aes256Gcm::new(key.into());

    // Extract nonce (first 12 bytes)
    let nonce = Nonce::from_slice(&encrypted[0..12]);

    // Extract ciphertext (remaining bytes)
    let ciphertext = &encrypted[12..];

    // Decrypt
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;

    String::from_utf8(plaintext).context("Decrypted data is not valid UTF-8")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;

    static INIT: Once = Once::new();

    fn init_test_key() {
        INIT.call_once(|| {
            init_encryption_key(Some("test-key-for-testing")).unwrap();
        });
    }

    #[test]
    fn test_encrypt_decrypt() {
        init_test_key();

        let original = r#"{"schedule": "test data"}"#;
        let encrypted = encrypt_data(original).unwrap();
        let decrypted = decrypt_data(&encrypted).unwrap();

        assert_eq!(original, decrypted);
    }

    #[test]
    fn test_multiple_encryptions_different_output() {
        init_test_key();

        let original = r#"{"schedule": "test"}"#;
        let encrypted1 = encrypt_data(original).unwrap();
        let encrypted2 = encrypt_data(original).unwrap();

        // Should be different due to random nonce
        assert_ne!(encrypted1, encrypted2);

        // But both should decrypt to same value
        assert_eq!(decrypt_data(&encrypted1).unwrap(), original);
        assert_eq!(decrypt_data(&encrypted2).unwrap(), original);
    }
}
