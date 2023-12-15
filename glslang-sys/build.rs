use cmake::Config;
pub fn main() {
    let cmake_dst = Config::new("native/glslang")
        .configure_arg("-DENABLE_OPT=0")
        .configure_arg("-DENABLE_HLSL=0")
        .configure_arg("-DENABLE_GLSLANG_BINARIES=0")
        .profile("Release")
        .build_target("glslang")
        .build();

    let object_dst = cmake_dst.join(format!("build/glslang/Release"));

    println!("cargo:rustc-link-search=native={}", object_dst.display());

    println!("cargo:rustc-link-lib=static=glslang");
    println!("cargo:rustc-link-lib=static=GenericCodeGen");
    println!("cargo:rustc-link-lib=static=MachineIndependent");


}
