fn main() {
    if cfg!(windows) {
        println!("cargo:rustc-link-search=C:\\Windows\\system32");
    }
}

