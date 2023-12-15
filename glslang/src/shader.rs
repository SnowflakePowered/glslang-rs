use crate::ctypes::{ResourceType, ShaderOptions};
use crate::error::GlslangError;
use crate::error::GlslangError::ParseError;
use crate::input::ShaderInput;
use glslang_sys as sys;
use std::ffi::{CStr, CString};
use std::ptr::NonNull;
use crate::Compiler;

pub struct Shader(NonNull<sys::glslang_shader_t>);

impl Shader {
    pub fn new(compiler: &Compiler, input: ShaderInput) -> Result<Self, GlslangError> {
        let shader = Self(unsafe {
            NonNull::new(sys::glslang_shader_create(&input.input))
                .expect("glslang created null shader")
        });

        unsafe {
            if sys::glslang_shader_preprocess(shader.0.as_ptr(), &input.input) == 0 {
                return Err(ParseError(shader.get_log()));
            }
        }

        unsafe {
            if sys::glslang_shader_parse(shader.0.as_ptr(), &input.input) == 0 {
                return Err(ParseError(shader.get_log()));
            }
        }
        Ok(shader)
    }

    pub fn preamble(&mut self, preamble: String) -> Result<(), GlslangError> {
        let cstr = CString::new(preamble)?;
        unsafe {
            sys::glslang_shader_set_preamble(self.0.as_ptr(), cstr.as_ptr());
        }
        Ok(())
    }

    pub fn shift_binding(&mut self, resource_type: ResourceType, base: u32) {
        unsafe {
            sys::glslang_shader_shift_binding(self.0.as_ptr(), resource_type, base);
        }
    }

    pub fn shift_binding_for_set(&mut self, resource_type: ResourceType, base: u32, set: u32) {
        unsafe {
            sys::glslang_shader_shift_binding_for_set(self.0.as_ptr(), resource_type, base, set);
        }
    }

    pub fn options(&mut self, options: ShaderOptions) {
        unsafe { sys::glslang_shader_set_options(self.0.as_ptr(), options.0) }
    }

    // todo: make this version enum
    pub fn glsl_version(&mut self, version: i32) {
        unsafe { sys::glslang_shader_set_glsl_version(self.0.as_ptr(), version) }
    }

    fn get_log(&self) -> String {
        let c_str = unsafe { CStr::from_ptr(sys::glslang_shader_get_info_log(self.0.as_ptr())) };

        let string = CString::from(c_str)
            .into_string()
            .expect("Expected glslang info log to be valid UTF-8");

        string
    }

    fn get_code(&self) -> String {
        let c_str = unsafe { CStr::from_ptr(sys::glslang_shader_get_preprocessed_code(self.0.as_ptr())) };

        let string = CString::from(c_str)
            .into_string()
            .expect("Expected glslang info log to be valid UTF-8");

        string
    }

}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { sys::glslang_shader_delete(self.0.as_ptr()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ctypes::ShaderStage;
    use crate::input::ShaderSource;
    use crate::limits::ResourceLimits;

    #[test]
    pub fn test_parse() {
        let compiler = Compiler::acquire()
            .unwrap();

        let source = ShaderSource::try_from(String::from(
            r#"
#version 450

layout(location = 0) out vec4 color;
layout(binding = 1) uniform sampler2D tex;

void main() {
    color = texture(tex, vec2(0.0));
}
        "#,
        ))
        .expect("source");

        let limits = ResourceLimits::default();
        let input = ShaderInput::new(&source, &limits, ShaderStage::GLSLANG_STAGE_FRAGMENT);
        let shader = Shader::new(&compiler, input).expect("shader init");

        let code = shader.get_code();
        println!("{}", code);
    }
}
