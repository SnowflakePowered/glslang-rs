use crate::ctypes::ShaderStage;
use crate::error::GlslangError;
use crate::limits::ResourceLimits;
use crate::{EnvVersion, GlslProfile, SourceLanguage, SpirvVersion, TargetEnv};
use glslang_sys as sys;
use glslang_sys::glsl_include_callbacks_s;
use std::ffi::CString;

pub struct ShaderSource(CString);
impl TryFrom<String> for ShaderSource {
    type Error = GlslangError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(CString::new(value)?))
    }
}

pub struct ShaderInput<'a> {
    // Keep these alive.
    _source: &'a ShaderSource,
    _resource: &'a sys::glslang_resource_t,
    pub(crate) input: sys::glslang_input_t,
}

#[derive(Debug, Copy, Clone)]
pub struct CompilerOptions {
    pub source_language: SourceLanguage,
    pub target: TargetEnv,
    pub target_version: EnvVersion,
    pub spirv_version: SpirvVersion,
    pub version_profile: Option<(i32, GlslProfile)>,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        Self {
            source_language: SourceLanguage::GLSL,
            target: TargetEnv::Vulkan,
            target_version: EnvVersion::Vulkan1_0,
            spirv_version: SpirvVersion::SPIRV_1_0,
            version_profile: None,
        }
    }
}

impl<'a> ShaderInput<'a> {
    pub fn new(
        source: &'a ShaderSource,
        resource: &'a ResourceLimits,
        stage: ShaderStage,
        options: &CompilerOptions,
    ) -> Self {
        Self {
            _source: source,
            _resource: &resource.0,
            input: sys::glslang_input_t {
                language: options.source_language,
                stage,
                client: options.target,
                client_version: options.target_version,
                target_language: sys::glslang_target_language_t::SPIRV,
                target_language_version: options.spirv_version,
                code: source.0.as_ptr(),
                default_version: options.version_profile.map_or(100, |o| o.0),
                default_profile: options.version_profile.map_or(GlslProfile::None, |o| o.1),
                force_default_version_and_profile: options.version_profile.map_or(0, |_| 1),
                forward_compatible: 0,
                messages: sys::glslang_messages_t::DEFAULT,
                resource: &resource.0,
                callbacks: glsl_include_callbacks_s {
                    include_system: None,
                    include_local: None,
                    free_include_result: None,
                },
                callbacks_ctx: core::ptr::null_mut(),
            },
        }
    }
}
