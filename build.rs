use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let raylib_out_dir: String = env::var("OUT_DIR").unwrap();
    let dst = cmake::build("raylib");
    println!("raylib: {}", raylib_out_dir);
    cc::Build::new()
        .files(vec!["binding/rgui.c"])
        .include("binding")
        .include("raygui/src")
        .include(format!("{raylib_out_dir}/include"))
        .warnings(false)
        .extra_warnings(false)
        .compile("rgui");

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=raylib");

    // Link to the library
    println!("cargo:rustc-link-search=native={raylib_out_dir}/lib");
    println!("cargo:rustc-link-lib=static=raylib");

    Ok(())
}
