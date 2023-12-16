# clockery (rust)
A fully rewritten Clockery in Rust programming language for performance boost and fun reasons

## Building (Linux only)

I still can't find a way to make rust-sdl2 to statically link libsdl2-ttf and libsdl2 together god damn it. So stuck with dynamic linking until I found a solution.

1. Install `libsdl2-dev` and `libsdl2-ttf-dev` package using something like apt
2. Run `cargo build -r` for building it in release mode. Mind you that it still contain debug information and you can turn it off in `Cargo.toml`
