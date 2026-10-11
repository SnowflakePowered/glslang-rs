use glob::glob;
use sha2::{Digest, Sha256};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn add_subdirectory(build: &mut cc::Build, directory: &str) {
    for entry in
        glob(&*format!("native/glslang/{directory}/**/*.cpp")).expect("failed to read glob")
    {
        if let Ok(path) = entry {
            build.file(path);
        }
    }

    for entry in glob(&*format!("native/glslang/{directory}/**/*.c")).expect("failed to read glob")
    {
        if let Ok(path) = entry {
            build.file(path);
        }
    }
}

const WASI_SDK_VERSION: &str = "34.0";

/// Locate wasi-sdk, downloading it into `OUT_DIR` if it is not provided through `WASI_SDK_PATH`.
fn wasi_sdk() -> PathBuf {
    println!("cargo:rerun-if-env-changed=WASI_SDK_PATH");
    if let Some(path) = env::var_os("WASI_SDK_PATH") {
        return PathBuf::from(path);
    }

    // (platform, sha256 of wasi-sdk-{WASI_SDK_VERSION}-{platform}.tar.gz)
    let (platform, sha256) = match (env::consts::ARCH, env::consts::OS) {
        ("x86_64", "linux") => ("x86_64-linux", "b761e3a0721dbae9c09a0059e5fdb2bf917d1b4a8a7b430fb3b5aafb0984b2c4"),
        ("aarch64", "linux") => ("arm64-linux", "f7e243dff54d60bcc576e94d6166b69f410f2500ae4a9ceef34315be10e77971"),
        ("x86_64", "macos") => ("x86_64-macos", "87d27fa8adc68dee59bfbf2e22a6d34ef717c34d6bf1d8af2a56fc929d9ce0eb"),
        ("aarch64", "macos") => ("arm64-macos", "9c59398106b417f8f14913380fdf0097a8cc0ff4af9eb3ce0065a859e88d49e9"),
        ("x86_64", "windows") => ("x86_64-windows", "cccb5c323a9b34f0349a9b09e8804a0a7632c68c3310f4b5f437ed57d7e71d8f"),
        ("aarch64", "windows") => ("arm64-windows", "45e1c71f3e965621e7b98ebe1d37b0e4b1f77f3e8072113ffb4534e67b1a4b7c"),
        (arch, os) => panic!(
            "no wasi-sdk download for host {arch}-{os}; install wasi-sdk and set WASI_SDK_PATH"
        ),
    };

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let name = format!("wasi-sdk-{WASI_SDK_VERSION}-{platform}");
    let sdk = out_dir.join(&name);
    if sdk.join("share/wasi-sysroot").is_dir() {
        return sdk;
    }

    let major = WASI_SDK_VERSION.split('.').next().unwrap();
    let url = format!(
        "https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-{major}/{name}.tar.gz"
    );
    let archive = out_dir.join(format!("{name}.tar.gz"));
    println!("cargo:warning=Downloading {url}");
    let status = Command::new("curl")
        .args(["--fail", "--location", "--silent", "--show-error", "--output"])
        .arg(&archive)
        .arg(&url)
        .status()
        .expect("failed to run curl");
    assert!(status.success(), "downloading wasi-sdk failed with {status}");

    let digest = Sha256::digest(fs::read(&archive).expect("failed to read wasi-sdk archive"));
    let digest: String = digest.iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(digest, sha256, "wasi-sdk archive checksum mismatch");

    let status = Command::new("tar")
        .arg("-xzf")
        .arg(&archive)
        .arg("-C")
        .arg(&out_dir)
        .status()
        .expect("failed to run tar");
    assert!(status.success(), "extracting wasi-sdk failed with {status}");
    let _ = fs::remove_file(&archive);
    sdk
}

pub fn main() {
    if env::var("DOCS_RS").is_ok() {
        println!("cargo:warning=Skipping glslang native build for docs.rs.");
        return;
    }

    println!("cargo:rerun-if-changed=native/glslang");
    println!("cargo:rerun-if-changed=native/build_info");

    let mut glslang_build = cc::Build::new();
    glslang_build
        .cpp(true)
        .std("c++17")
        .define("ENABLE_SPIRV", "ON")
        .define("ENABLE_HLSL", "ON")
        .define("ENABLE_OPT", "OFF")
        .define("ENABLE_GLSLANG_BINARIES", "OFF")
        .includes(&["native/glslang", "native/build_info"]);

    let wasm_unknown = env::var("TARGET").is_ok_and(|t| t == "wasm32-unknown-unknown");
    let wasi_sysroot = wasm_unknown.then(|| {
        let sdk = wasi_sdk();
        let sysroot = sdk.join("share/wasi-sysroot");
        let exe = if cfg!(windows) { ".exe" } else { "" };
        glslang_build
            .compiler(sdk.join(format!("bin/clang++{exe}")))
            .archiver(sdk.join(format!("bin/llvm-ar{exe}")))
            .cpp_link_stdlib(None)
            .flag("--target=wasm32-wasip1")
            .flag(&format!("--sysroot={}", sysroot.display()))
            .flag("-fno-exceptions");
        sysroot
    });

    add_subdirectory(&mut glslang_build, "glslang/CInterface");
    add_subdirectory(&mut glslang_build, "glslang/GenericCodeGen");
    add_subdirectory(&mut glslang_build, "glslang/HLSL");
    add_subdirectory(&mut glslang_build, "glslang/MachineIndependent");

    add_subdirectory(&mut glslang_build, "SPIRV");

    glslang_build.compile("glslang");
    println!("cargo:rustc-link-lib=static=glslang");

    if let Some(sysroot) = wasi_sysroot {
        let lib = sysroot.join("lib/wasm32-wasip1");
        println!("cargo:rustc-link-search=native={}", lib.join("noeh").display());
        println!("cargo:rustc-link-search=native={}", lib.display());
        println!("cargo:rustc-link-lib=static=c++");
        println!("cargo:rustc-link-lib=static=c++abi");
        println!("cargo:rustc-link-lib=static=c");
    }
}
