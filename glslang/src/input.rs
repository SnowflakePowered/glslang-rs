use crate::ctypes::ShaderStage;
use crate::error::GlslangError;
use crate::limits::ResourceLimits;
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
    source: &'a ShaderSource,
    resource: &'a sys::glslang_resource_t,
    pub(crate) input: sys::glslang_input_t,
}

impl<'a> ShaderInput<'a> {
    pub fn new(source: &'a ShaderSource, resource: &'a ResourceLimits, stage: ShaderStage) -> Self {
        Self {
            source,
            resource: &resource.0,
            input: sys::glslang_input_t {
                language: sys::glslang_source_t_GLSLANG_SOURCE_GLSL,
                stage,
                client: sys::glslang_client_t::GLSLANG_CLIENT_NONE,
                client_version: sys::glslang_target_client_version_t::GLSLANG_TARGET_VULKAN_1_0,
                target_language: sys::glslang_target_language_t::GLSLANG_TARGET_NONE,
                target_language_version:
                    sys::glslang_target_language_version_t::GLSLANG_TARGET_SPV_1_0,
                code: source.0.as_ptr(),
                default_version: 100,
                default_profile: sys::glslang_profile_t::GLSLANG_NO_PROFILE,
                force_default_version_and_profile: 0,
                forward_compatible: 0,
                messages: sys::glslang_messages_t::GLSLANG_MSG_DEFAULT_BIT,
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
