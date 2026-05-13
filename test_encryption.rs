use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hostname::get;

const ENCRYPTION_MAGIC: &[u8] = b"GHOSTWRITER_V1";

fn get_machine_key() -> Vec<u8> {
    let hostname = get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "ghostwriter".to_string());
    hostname.as_bytes().to_vec()
}

fn encrypt_api_key(api_key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let machine_key = get_machine_key();
    let encrypted: Vec<u8> = api_key
        .as_bytes()
        .iter()
        .enumerate()
        .map(|(i, b)| b ^ machine_key[i % machine_key.len()])
        .collect();

    let mut result = ENCRYPTION_MAGIC.to_vec();
    result.extend(encrypted);
    Ok(BASE64.encode(&result))
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
    let api_key = "YOUR_API_KEY_HERE";
    match encrypt_api_key(api_key) {
        Ok(encrypted) => {
            println!("Encrypted: {}", encrypted);

            match decrypt_api_key(&encrypted) {
                Ok(decrypted) => {
                    println!("Decrypted: {}", decrypted);
                    println!("Match: {}", decrypted == api_key);
                }
                Err(e) => eprintln!("Decryption error: {}", e),
            }
        }
        Err(e) => eprintln!("Encryption error: {}", e),
    }

    let machine_key = get_machine_key();
    println!("Machine key: {:?}", machine_key);
    println!("Machine key as string: {}", String::from_utf8_lossy(&machine_key));
}
