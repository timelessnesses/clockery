fn main() {
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,@loader_path");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN");

    println!("cargo:rustc-link-lib=png");

    // println!("cargo:rerun-if-env-changed=SDL2_TTF_LIBRARY_PATH");
    // if let Ok(library_path) = std::env::var("SDL2_TTF_LIBRARY_PATH") {
    //     println!("cargo:rustc-link-search=native={}", library_path);
    // }
}
