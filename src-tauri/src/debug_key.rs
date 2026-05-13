use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use hostname::get;

const ENCRYPTION_MAGIC: &[u8] = b"GHOSTWRITER_V1";

fn get_machine_key_from_hostname() -> Vec<u8> {
    get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "ghostwriter".to_string())
        .as_bytes()
        .to_vec()
}

fn encrypt_api_key(api_key: &str) -> Result<String, String> {
    let machine_key = get_machine_key_from_hostname();
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

fn decrypt_api_key(encrypted: &str) -> Result<String, String> {
    let data = BASE64.decode(encrypted)
        .map_err(|e| format!("Base64 decode error: {}", e))?;

    if data.starts_with(ENCRYPTION_MAGIC) {
        let encrypted_bytes = &data[ENCRYPTION_MAGIC.len()..];
        let machine_key = get_machine_key_from_hostname();
        let decrypted: Vec<u8> = encrypted_bytes
            .iter()
            .enumerate()
            .map(|(i, b)| b ^ machine_key[i % machine_key.len()])
            .collect();
        String::from_utf8(decrypted)
            .map_err(|e| format!("UTF-8 error: {}", e))
    } else {
        Err("Invalid key format".to_string())
    }
}

fn main() {
    println!("=== Debugging Key Encryption ===");

    let hostname_result = get();
    println!("hostname::get() result: {:?}", hostname_result);
    let hostname_str = hostname_result
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    println!("Hostname string: '{}'", hostname_str);

    let machine_key = get_machine_key_from_hostname();
    println!("Machine key bytes: {:?}", machine_key);
    println!("Machine key as string: '{}'", String::from_utf8_lossy(&machine_key));

    let api_key = "YOUR_API_KEY_HERE";
    println!("\nOriginal API key: {}", api_key);

    let encrypted = encrypt_api_key(api_key);
    println!("Encrypted: {}", encrypted);

    match encrypted {
        Ok(ref e) => {
            match decrypt_api_key(e) {
                Ok(decrypted) => {
                    println!("Decrypted: {}", decrypted);
                    println!("Match: {}", decrypted == api_key);
                    if decrypted != api_key {
                        println!("ERROR: Decryption failed!");
                    }
                }
                Err(e) => println!("Decryption error: {:?}", e),
            }
        }
        Err(e) => println!("Encryption error: {:?}", e),
    }
}
