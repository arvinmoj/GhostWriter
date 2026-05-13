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

    println!("\n=== Environment Variables ===");
    if let Some(host) = env::var_os("HOST") {
        println!("HOST = {:?}", host);
    }
    if let Some(hostname) = env::var_os("HOSTNAME") {
        println!("HOSTNAME = {:?}", hostname);
    }
    if let Some(localhostname) = env::var_os("LOCALHOSTNAME") {
        println!("LOCALHOSTNAME = {:?}", localhostname);
    }

    let machine_key = get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "ghostwriter".to_string());
    println!("\nConfig machine key: \"{}\"", machine_key);
    println!("As bytes: {:?}", machine_key.as_bytes());
}
