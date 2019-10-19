use std::env;

fn main() {
    if let Some(_) = env::var_os("COMPILING_UNDER_CARGO_WEB") {
        println!("cargo:rustc-cfg=feature=\"frontend\"");
    } else {
        println!("cargo:rustc-cfg=feature=\"backend\"");
    }

    println!("cargo:warning={:?}", env::var_os("COMPILING_UNDER_CARGO_WEB"));
}
