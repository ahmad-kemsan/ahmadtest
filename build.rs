fn main() {
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=dylib=LexActivator");
}

// use std::env;
// use std::path::PathBuf;

// fn main() {
//     // Set the path to the directory containing the static library
//     let library_dir = ".";

//     // Specify the library name (without the lib prefix or file extension)
//     let library_name = "LexActivator";

//     // Set the target directory where the compiled binary will be placed
//     let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
//     let target_dir = out_dir.join("target");
//     let _ = std::fs::create_dir_all(&target_dir);

//     // Link against the static library
//     println!("cargo:rustc-link-search={}", library_dir);
//     println!("cargo:rustc-link-lib=static={}", library_name);

//     // frameworks
//     println!("cargo:rustc-link-lib=framework=CoreFoundation");
//     println!("cargo:rustc-link-lib=framework=Security");
//     println!("cargo:rustc-link-lib=framework=Foundation");


//     // Set the target directory as the place to put the resulting binary
//     println!("cargo:rustc-link-search={}", target_dir.display());
// }