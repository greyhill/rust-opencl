fn main() {
    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-search=C:\\Windows\\system32");
}

