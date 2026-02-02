use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng, Result},
};
use sha2::{Digest, Sha256};

const NONCE_SIZE: usize = 12;

fn derive_key(passphrase: &str) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(passphrase.as_bytes());
    hasher.finalize().into()
}

pub fn encrypt(content: &[u8], passphrase: &str) -> Result<Vec<u8>> {
    let derived_key = derive_key(passphrase);
    let key: &Key<Aes256Gcm> = (&derived_key).into();
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, content)?;

    let mut result = nonce.to_vec();
    result.extend(ciphertext);
    Ok(result)
}

pub fn decrypt(encrypted: &[u8], passphrase: &str) -> Result<Vec<u8>> {
    if encrypted.len() < NONCE_SIZE {
        return Err(aes_gcm::aead::Error);
    }

    let derived_key = derive_key(passphrase);
    let key: &Key<Aes256Gcm> = (&derived_key).into();
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&encrypted[..NONCE_SIZE]);
    let ciphertext = &encrypted[NONCE_SIZE..];
    let plaintext = cipher.decrypt(nonce, ciphertext)?;

    Ok(plaintext)
}
