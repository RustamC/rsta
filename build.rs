use std::env;
use std::path::Path;

fn main() -> miette::Result<()> {
    let wrapper_path = std::path::PathBuf::from("./");

    let sta_path = env::var("OPENSTA_DIR").unwrap_or("./opensta".to_string());
    let sta_path = std::path::PathBuf::from(sta_path)
        .join("include")
        .join("sta");

    if !sta_path.exists() {
        eprintln!(
            "{} is not a directory! Set OPENSTA_DIR environmnet variable!",
            sta_path.display()
        );
    }

    let mut sta_bind =
        autocxx_build::Builder::new("src/bridge.rs", &[&wrapper_path, &sta_path]).build()?;
    sta_bind.flag_if_supported("-std=c++17").compile("sta");

    println!("cargo:rerun-if-changed=src/bridge.rs");

    // OpenSTA
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-lib=OpenSTA");
    println!(
        "cargo:rustc-link-search={}",
        Path::new(&dir).join("opensta").join("lib").display()
    );

    // Tcl
    println!("cargo:rustc-link-lib=static=tcl");
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu/");

    println!("cargo:rustc-link-lib=static=z");
    Ok(())
}
