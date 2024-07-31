include!(concat!(env!("OUT_DIR"), "/hello.rs"));

fn main() {
    println!("Main is calling message() = {}", message());
}
