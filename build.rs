use std::env;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=dylib=LexActivator");
    println!("cargo:rustc-link-arg=-Wl,-rpath,{}", out_dir);

}