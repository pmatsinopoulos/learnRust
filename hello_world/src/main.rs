use regex::Regex;

fn main() {
    println!("Hello, world!");
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2024-01-01"));
    cfg_if::cfg_if! {
        if #[cfg(feature = "foo")] {
            println!("foo");
        }
    }
    // example of reading an environment variable:
    let version = env!("CARGO_PKG_VERSION");
    println!("Version = {}", version);
}
