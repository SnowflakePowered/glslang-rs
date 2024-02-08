#[doc(hidden)]
pub use glslang_sys::glslang_resource_type_t as ResourceType;

/// GLSL profiles.
pub use glslang_sys::glslang_profile_t as GlslProfile;
/// Shader compilation flags.
pub use glslang_sys::glslang_shader_options_t as ShaderOptions;
/// The language of the source file.
pub use glslang_sys::glslang_source_t as SourceLanguage;
/// Shader stage mask.
pub use glslang_sys::glslang_stage_mask_t as ShaderStages;
/// Shader stages for the input.
pub use glslang_sys::glslang_stage_t as ShaderStage;
/// SPIR-V language versions.
pub use glslang_sys::glslang_target_language_version_t as SpirvVersion;
