use ghostwriter_lib::config::{encrypt_api_key, decrypt_api_key};

fn main() {
    let api_key = "YOUR_API_KEY_HERE";
    println!("Original API key: {}", api_key);

    match encrypt_api_key(api_key) {
        Ok(encrypted) => {
            println!("Encrypted: {}", encrypted);

            match decrypt_api_key(&encrypted) {
                Ok(decrypted) => {
                    println!("Decrypted: {}", decrypted);
                    println!("Match: {}", decrypted == api_key);
                }
                Err(e) => println!("Decryption error: {:?}", e),
            }
        }
        Err(e) => println!("Encryption error: {:?}", e),
    }
}
