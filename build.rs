use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut b = autocxx_build::Builder::new(
        "src/lib.rs",
        &[
            "./node/deps/openssl/config/",
            "./node/deps/openssl/openssl/include/",
            "./node/deps/uv/include/",
            "./node/deps/v8/include/",
            "./node/deps/zlib/",
            "./node/src/",
        ],
    )
    .extra_clang_args(&["-std=c++20"])
    .build()?;
    b.flag_if_supported("-std=c++20").compile("node");
    println!("cargo::rerun-if-changed=./src/lib.rs");
    println!("cargo::rustc-link-search=native=./node/out/Release");
    println!("cargo::rustc-link-lib=dylib=node");
    Ok(())
}
