use anyhow::Result;
use rand::RngCore;

fn main() -> Result<()> {
    let mut key = [0u8; 64]; // 512 bits = 64 bytes
    rand::thread_rng().fill_bytes(&mut key);
    println!("\nGenerated key for HMAC:\n{key:?}");

    let b64u_key = base64_url::encode(&key);
    println!("Base64 URL encoded key:\n{b64u_key}");

    Ok(())
}
