// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=lib.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("lib.c")
        .compile("mylib");
        
    println!("cargo:rustc-link-search=all=/Users/jonathan/Documents/test/rust/playground_ffi");
    // println!("cargo:rustc-link-search=all=/Users/jonathan/Documents/test/rust/playground_ffi/target/debug");
    println!("cargo:rustc-link-lib=mylib");
}
