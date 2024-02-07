use cmake::Config;
pub fn main() {
    let glslang_dst = Config::new("native/glslang")
        .configure_arg("-DENABLE_OPT=OFF")
        .configure_arg("-DENABLE_HLSL=ON")
        .configure_arg("-DENABLE_GLSLANG_BINARIES=OFF")
        .profile("Release")
        .build_target("glslang")
        .build();

    let spirv_dst = Config::new("native/glslang")
        .configure_arg("-DENABLE_OPT=OFF")
        .configure_arg("-DENABLE_HLSL=ON")
        .configure_arg("-DENABLE_GLSLANG_BINARIES=OFF")
        .profile("Release")
        .build_target("SPIRV")
        .build();

    let glslang_dst = glslang_dst.join(format!("build/glslang/Release"));
    let spirv_dst = spirv_dst.join(format!("build/SPIRV/Release"));

    println!("cargo:rustc-link-search=native={}", glslang_dst.display());
    println!("cargo:rustc-link-search=native={}", spirv_dst.display());

    println!("cargo:rustc-link-lib=static=glslang");

    println!("cargo:rustc-link-lib=static=GenericCodeGen");
    println!("cargo:rustc-link-lib=static=MachineIndependent");
    println!("cargo:rustc-link-lib=static=SPIRV");
}
