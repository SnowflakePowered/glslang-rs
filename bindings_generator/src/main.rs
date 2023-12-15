use std::env;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("glslang-sys/native/wrapper.h")
        .rustified_enum("glslang_stage_t")
        .bitfield_enum("glslang_stage_mask_t")
        .rustified_enum("glslang_client_t")
        .rustified_enum("glslang_target_language_t")
        .rustified_enum("glslang_target_client_version_t")
        .rustified_enum("glslang_target_language_version_t")
        .rustified_enum("glslang_optimization_level_t")
        .rustified_enum("glslang_texture_sampler_transform_mode_t")
        .bitfield_enum("glslang_messages_t")
        .bitfield_enum("glslang_reflection_options_t")
        .bitfield_enum("glslang_profile_t")
        .bitfield_enum("glslang_shader_options_t")
        .rustified_enum("glslang_resource_type_t")
        .rustified_enum("glslang_profile_t")
        .clang_arg("-Iglslang-sys/native/glslang/glslang/Include")
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(
            env::current_dir()
                .unwrap()
                .join("glslang-sys/src/bindings.rs"),
        )
        .expect("Couldn't write bindings!");
}
