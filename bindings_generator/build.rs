fn main() {
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap()
    );
    println!("cargo:rustc-env=HOST={}", std::env::var("HOST").unwrap());
    println!(
        "cargo:rustc-env=OUT_DIR={}",
        std::env::var("OUT_DIR").unwrap()
    );
}
