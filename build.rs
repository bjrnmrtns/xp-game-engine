
extern crate cmake;

use cmake::Config;

fn main()
{
    let dst = Config::new("windowing").build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=dylib=c++");
    println!("cargo:rustc-link-lib=SDL2");
    println!("cargo:rustc-link-lib=static=windowing");
}
