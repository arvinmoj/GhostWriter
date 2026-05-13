use hostname::get;
use std::env;

fn main() {
    println!("=== Hostname Debug ===");

    match get() {
        Ok(hostname) => {
            let hostname_str = hostname.to_string_lossy().to_string();
            println!("hostname::get() = Ok(\"{}\")", hostname_str);

            let bytes = hostname.as_bytes();
            println!("Bytes: {:?}", bytes);
            println!("Bytes as string: {}", String::from_utf8_lossy(bytes));
        }
        Err(e) => {
            println!("hostname::get() = Err({:?})", e);
        }
    }
}
