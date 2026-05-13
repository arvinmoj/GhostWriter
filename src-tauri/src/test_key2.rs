use ghostwriter_lib::config::get_machine_key;
use hostname;

fn main() {
    let machine_key = get_machine_key();
    println!("Machine key bytes: {:?}", machine_key);
    println!("Machine key as string: {}", String::from_utf8_lossy(&machine_key));

    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    println!("Hostname: {}", hostname);
}
