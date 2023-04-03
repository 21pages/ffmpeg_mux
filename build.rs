use cc::Build;
use std::{
    env,
    path::{Path, PathBuf},
};

fn main() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=ffmpeg");
    bindgen::builder()
        .header("src/ffi.h")
        .rustified_enum("*")
        .generate()
        .unwrap()
        .write_to_file(Path::new(&env::var_os("OUT_DIR").unwrap()).join("mux_ffi.rs"))
        .unwrap();

    let mut builder = Build::new();

    // system
    #[cfg(target_os = "windows")]
    ["User32", "bcrypt", "ole32"].map(|lib| println!("cargo:rustc-link-lib={}", lib));

    // ffmpeg
    let ffmpeg_dir = manifest_dir.join("ffmpeg");
    #[cfg(windows)]
    let ffmpeg_path = ffmpeg_dir.join("windows").join("x64");
    #[cfg(target_os = "linux")]
    let ffmpeg_path = ffmpeg_dir.join("linux").join("x64");
    builder.include(ffmpeg_path.join("include"));
    println!(
        "cargo:rustc-link-search=native={}",
        ffmpeg_path.join("lib").display()
    );
    ["avcodec", "avutil", "avformat"].map(|lib| println!("cargo:rustc-link-lib=static={}", lib));

    // crate
    builder.file("src/mux.c").cpp(false).compile("mux");
}
