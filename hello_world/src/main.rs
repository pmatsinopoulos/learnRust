use regex::Regex;
use std::env;

// Declare the external function
extern "C" {
    fn hello();
}

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
    // This environment variable is available at compile time.
    let version = env!("CARGO_PKG_VERSION");
    println!("Version = {}", version);

    // There are environment variables which are not available at compile time, but
    // only at run time. So, the following code does not compile
    //
    // let foo_bar = env!("FOO_BAR"); //<-------- does not compile
    //
    // The proper way to read this is:
    let foo_bar = env::var("FOO_BAR");
    println!("FOO_BAR *{:?}*", foo_bar);

    // Call the C function
    unsafe {
        hello();
    }
}
