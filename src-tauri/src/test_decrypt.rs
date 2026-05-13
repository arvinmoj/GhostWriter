use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hostname::get;

const ENCRYPTION_MAGIC: &[u8] = b"GHOSTWRITER_V1";

fn get_machine_key() -> Vec<u8> {
    let hostname = get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "ghostwriter".to_string());
    hostname.as_bytes().to_vec()
}

fn decrypt_api_key(encrypted: &str) -> Result<String, Box<dyn std::error::Error>> {
    let data = BASE64.decode(encrypted)
        .map_err(|e| format!("Base64 decode error: {}", e))?;

    if data.starts_with(ENCRYPTION_MAGIC) {
        let encrypted_bytes = &data[ENCRYPTION_MAGIC.len()..];
        let machine_key = get_machine_key();
        let decrypted: Vec<u8> = encrypted_bytes
            .iter()
            .enumerate()
            .map(|(i, b)| b ^ machine_key[i % machine_key.len()])
            .collect();
        String::from_utf8(decrypted)
            .map_err(|e| format!("UTF-8 conversion error: {}", e))
    } else {
        Err("Invalid encrypted key format".into())
    }
}

fn main() {
    let encrypted = "R0hPU1RXUklURVJfVjEeCk4NHUIdH0FeAVkNX1dQUlpYCB4KXQcEXwxZBlFdX10dCF9RAllUBVBVXA0ISgkLVFYPVQUHAwpeDUtZDQZZDQlZU1ELWVxP";
    println!("Trying to decrypt: {}", encrypted);

    match decrypt_api_key(encrypted) {
        Ok(decrypted) => {
            println!("SUCCESS: Decrypted to: {}", decrypted);
        }
        Err(e) => {
            println!("ERROR: {}", e);
        }
    }

    println!("\nDebug info:");
    let machine_key = get_machine_key();
    println!("Machine key: {:?}", machine_key);
    println!("Machine key as string: {}", String::from_utf8_lossy(&machine_key));

    let data = BASE64.decode(encrypted).unwrap();
    println!("Decoded data length: {}", data.len());
    println!("First 20 bytes: {:?}", &data[..20.min(data.len())]);
    println!("Magic bytes: {:?}", ENCRYPTION_MAGIC);
    println!("Starts with magic: {}", data.starts_with(ENCRYPTION_MAGIC));

    if data.starts_with(ENCRYPTION_MAGIC) {
        let encrypted_bytes = &data[ENCRYPTION_MAGIC.len()..];
        println!("Encrypted bytes length: {}", encrypted_bytes.len());
        println!("First 10 encrypted bytes: {:?}", &encrypted_bytes[..10.min(encrypted_bytes.len())]);
    }
}
