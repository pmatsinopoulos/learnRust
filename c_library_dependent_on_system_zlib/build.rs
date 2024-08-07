fn main() {
    let mut cfg = cc::Build::new();

    cfg.file("src/open_ssl_user.c"); // a file that depends on openssl library

    cfg.compile("open_ssl_user");

    println!("cargo::rerun-if-changed=src/open_ssl_user.c");
}
