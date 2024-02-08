use crate::callbacks::IncludeCallback;
use crate::ctypes::ShaderStage;
use crate::limits::ResourceLimits;
use crate::{callbacks, EnvVersion, GlslProfile, SourceLanguage, SpirvVersion, TargetEnv};
use glslang_sys as sys;
use glslang_sys::glsl_include_callbacks_s;
use std::ffi::{c_void, CString};

#[derive(Debug, Clone)]
pub struct ShaderSource(CString);

impl From<String> for ShaderSource {
    fn from(value: String) -> Self {
        // panic safety: String never has null bytes
        Self(CString::new(value).unwrap())
    }
}

impl From<&str> for ShaderSource {
    fn from(value: &str) -> Self {
        // panic safety: String never has null bytes
        Self(CString::new(value.to_string()).unwrap())
    }
}

#[derive(Clone)]
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
        callbacks: Option<IncludeCallback>,
    ) -> Self {
        let callbacks_ctx = callbacks.map_or(core::ptr::null_mut(), |callback| {
            Box::into_raw(Box::new(callback))
        });

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
                    include_system: Some(callbacks::_glslang_rs_sys_func),
                    include_local: Some(callbacks::_glslang_rs_local_func),
                    free_include_result: Some(callbacks::_glslang_rs_drop_result),
                },
                callbacks_ctx: callbacks_ctx as *mut c_void,
            },
        }
    }
}
