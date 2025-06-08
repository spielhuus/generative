use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let raylib_out_dir: String = env::var("OUT_DIR").unwrap();
    let dst = cmake::build("raylib");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=raylib");

    // Link to the library
    println!("cargo:rustc-link-search=native={raylib_out_dir}/lib");
    println!("cargo:rustc-link-lib=static=raylib");

    Ok(())
}
