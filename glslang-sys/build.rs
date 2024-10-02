use glob::glob;
use std::env;

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

    add_subdirectory(&mut glslang_build, "glslang/CInterface");
    add_subdirectory(&mut glslang_build, "glslang/GenericCodeGen");
    add_subdirectory(&mut glslang_build, "glslang/HLSL");
    add_subdirectory(&mut glslang_build, "glslang/MachineIndependent");

    add_subdirectory(&mut glslang_build, "SPIRV");

    glslang_build.compile("glslang");
    println!("cargo:rustc-link-lib=static=glslang");
}
