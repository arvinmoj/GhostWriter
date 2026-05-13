use hostname::get;

fn main() {
    println!("=== Debugging Key Encryption ===");

    let hostname_result = get();
    println!("hostname::get() result: {:?}", hostname_result);
    let hostname_str = hostname_result
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    println!("Hostname string: '{}'", hostname_str);
}
