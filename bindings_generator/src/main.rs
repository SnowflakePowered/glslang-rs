mod callbacks;

use std::env;

fn main() {
    let bindings = bindgen::Builder::default()
        .header("glslang-sys/native/wrapper.h")
        .parse_callbacks(Box::new(callbacks::GlslangCallbacks))
        .allowlist_type("glslang_.*")
        .allowlist_type("glsl_.*")
        .allowlist_function("glslang_.*")
        .blocklist_type("glslang_optimization_level_t")
        .blocklist_type("glslang_reflection_options_t") // currently not used
        .blocklist_type("glslang_executable_t") // currently not used
        .rustified_enum("glslang_stage_t")
        .bitfield_enum("glslang_stage_mask_t")
        .rustified_enum("glslang_client_t")
        .rustified_enum("glslang_target_language_t")
        .rustified_enum("glslang_target_client_version_t")
        .rustified_enum("glslang_target_language_version_t")
        .rustified_enum("glslang_texture_sampler_transform_mode_t")
        .bitfield_enum("glslang_messages_t")
        .bitfield_enum("glslang_reflection_options_t")
        .rustified_enum("glslang_profile_t")
        .rustified_enum("glslang_source_t")
        .bitfield_enum("glslang_shader_options_t")
        .rustified_enum("glslang_resource_type_t")
        .rustified_enum("glslang_profile_t")
        .no_default("glslang_limits_s")
        .clang_arg("-Iglslang-sys/native/glslang/glslang/Include")
        .clang_arg("-Iglslang-sys/native/glslang/glslang/Public")
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
