use hmac::{Hmac, Mac};
use sha1::Sha1;

fn hmac_sha1(key: &[u8], counter: &[u8]) -> anyhow::Result<[u8; 20]> {
    let mut hasher: Hmac<Sha1> = Mac::new_from_slice(key)?;

    hasher.update(counter);
    Ok(hasher.finalize().into_bytes().into())
}

fn hotp(key: &[u8], counter: u64) -> u32 {
    let hmac_result = hmac_sha1(key, &counter.to_be_bytes()).expect("hashing failed");

    let offset = (hmac_result[19] & 0x0f) as usize;

    let binary = ((hmac_result[offset] & 0x7f) as u32) << 24
        | (hmac_result[offset + 1] as u32) << 16
        | (hmac_result[offset + 2] as u32) << 8
        | (hmac_result[offset + 3] as u32);

    let modulo = 10_u32.pow(6);
    binary % modulo
}

pub fn totp(key: &[u8]) -> u32 {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let counter = timestamp / 30;

    hotp(&key, counter)
}
