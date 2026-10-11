use glob::glob;
use std::env;
use std::path::PathBuf;

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

/// Locate the installed wasi-sdk, which provides the C/C++ toolchain and libc/libc++ sysroot
/// that glslang needs on `wasm32-unknown-unknown`.
fn wasi_sdk() -> PathBuf {
    println!("cargo:rerun-if-env-changed=WASI_SDK_PATH");
    let path = env::var_os("WASI_SDK_PATH").unwrap_or_else(|| {
        panic!(
            "building for wasm32-unknown-unknown requires wasi-sdk"
        )
    });
    let sdk = PathBuf::from(path);
    assert!(
        sdk.join("share/wasi-sysroot").is_dir(),
        "WASI_SDK_PATH ({}) does not contain share/wasi-sysroot",
        sdk.display()
    );
    sdk
}

pub fn main() {
    if env::var("DOCS_RS").is_ok() {
        println!("cargo:warning=Skipping glslang native build for docs.rs.");
        return;
    }

    println!("cargo:rerun-if-changed=native/glslang");
    println!("cargo:rerun-if-changed=native/build_info");
    println!("cargo:rerun-if-changed=native/wasi_stubs.cpp");

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
    if wasm_unknown {
        // wasi-libc's WASI imports are satisfied by weak stubs, as there is no host to import from.
        glslang_build.file("native/wasi_stubs.cpp");
    }
    let wasi_sysroot = wasm_unknown.then(|| {
        let sdk = wasi_sdk();
        let sysroot = sdk.join("share/wasi-sysroot");
        let exe = if cfg!(windows) { ".exe" } else { "" };
        glslang_build
            .compiler(sdk.join(format!("bin/clang++{exe}")))
            .archiver(sdk.join(format!("bin/llvm-ar{exe}")))
            .cpp_link_stdlib(None)
            .flag("--target=wasm32-wasip1")
            .flag(&format!("--sysroot={}", sysroot.display()));
        sysroot
    });

    if glslang_build.get_compiler().is_like_msvc() {
        glslang_build.flag("/EHs-c-").define("_HAS_EXCEPTIONS", "0");
    } else {
        glslang_build.flag("-fno-exceptions");
    }

    add_subdirectory(&mut glslang_build, "glslang/CInterface");
    add_subdirectory(&mut glslang_build, "glslang/GenericCodeGen");
    add_subdirectory(&mut glslang_build, "glslang/HLSL");
    add_subdirectory(&mut glslang_build, "glslang/MachineIndependent");

    add_subdirectory(&mut glslang_build, "SPIRV");

    glslang_build.compile("glslang");
    println!("cargo:rustc-link-lib=static=glslang");

    if let Some(sysroot) = wasi_sysroot {
        let lib = sysroot.join("lib/wasm32-wasip1");
        // glslang is built without exceptions, but links the exception-enabled runtime so that it
        // can share a single libc++ with other crates, such as spirv-cross-sys, that need exceptions.
        println!("cargo:rustc-link-search=native={}", lib.join("eh").display());
        println!("cargo:rustc-link-search=native={}", lib.display());
        println!("cargo:rustc-link-lib=static=c++");
        println!("cargo:rustc-link-lib=static=c++abi");
        println!("cargo:rustc-link-lib=static=unwind");
        println!("cargo:rustc-link-lib=static=c");
    }
}
