use std::error::Error;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let include_path = PathBuf::from("src");

    // This assumes all your C++ bindings are in lib.rs
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&include_path]).build()?;
    b.flag_if_supported("-std=c++14")
     .compile("autocxx-demo"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/main.rs");

    // Add instructions to link to any C++ libraries you need.

    Ok(())
}