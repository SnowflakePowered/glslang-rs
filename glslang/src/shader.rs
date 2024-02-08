use crate::ctypes::{ResourceType, ShaderOptions, ShaderStage};
use crate::error::GlslangError;
use crate::error::GlslangError::ParseError;
use crate::input::ShaderInput;
use crate::Compiler;
use glslang_sys as sys;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct Shader<'a> {
    pub(crate) handle: NonNull<sys::glslang_shader_t>,
    pub(crate) stage: ShaderStage,
    _compiler: &'a Compiler,
}

impl<'a> Shader<'a> {
    pub fn new(_compiler: &'a Compiler, input: ShaderInput) -> Result<Self, GlslangError> {
        let shader = Self {
            handle: unsafe {
                NonNull::new(sys::glslang_shader_create(&input.input))
                    .expect("glslang created null shader")
            },
            stage: input.input.stage,
            _compiler,
        };

        unsafe {
            if sys::glslang_shader_preprocess(shader.handle.as_ptr(), &input.input) == 0 {
                return Err(ParseError(shader.get_log()));
            }
        }

        unsafe {
            if sys::glslang_shader_parse(shader.handle.as_ptr(), &input.input) == 0 {
                return Err(ParseError(shader.get_log()));
            }
        }
        Ok(shader)
    }

    pub fn preamble(&mut self, preamble: String) -> Result<(), GlslangError> {
        let cstr = CString::new(preamble)?;
        unsafe {
            sys::glslang_shader_set_preamble(self.handle.as_ptr(), cstr.as_ptr());
        }
        Ok(())
    }

    pub fn shift_binding(&mut self, resource_type: ResourceType, base: u32) {
        unsafe {
            sys::glslang_shader_shift_binding(self.handle.as_ptr(), resource_type, base);
        }
    }

    pub fn shift_binding_for_set(&mut self, resource_type: ResourceType, base: u32, set: u32) {
        unsafe {
            sys::glslang_shader_shift_binding_for_set(
                self.handle.as_ptr(),
                resource_type,
                base,
                set,
            );
        }
    }

    pub fn options(&mut self, options: ShaderOptions) {
        unsafe { sys::glslang_shader_set_options(self.handle.as_ptr(), options.0) }
    }

    pub fn glsl_version(&mut self, version: i32) {
        unsafe { sys::glslang_shader_set_glsl_version(self.handle.as_ptr(), version) }
    }

    fn get_log(&self) -> String {
        let c_str =
            unsafe { CStr::from_ptr(sys::glslang_shader_get_info_log(self.handle.as_ptr())) };

        let string = CString::from(c_str)
            .into_string()
            .expect("Expected glslang info log to be valid UTF-8");

        string
    }

    /// Convenience method to compile this shader without linking to other shaders.
    pub fn compile(&self) -> Result<Vec<u32>, GlslangError> {
        let mut program = self._compiler.create_program();
        program.add_shader(&self);
        program.compile(self.stage)
    }

    pub fn get_preprocessed_code(&self) -> String {
        let c_str = unsafe {
            // SAFETY: for Shader to be initialized preprocessing + parsing had to be complete.
            CStr::from_ptr(sys::glslang_shader_get_preprocessed_code(
                self.handle.as_ptr(),
            ))
        };

        let string = CString::from(c_str)
            .into_string()
            .expect("Expected glslang info log to be valid UTF-8");

        string
    }
}

impl<'a> Drop for Shader<'a> {
    fn drop(&mut self) {
        unsafe { sys::glslang_shader_delete(self.handle.as_ptr()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ctypes::ShaderStage;
    use crate::input::{CompilerOptions, ShaderSource};
    use crate::limits::ResourceLimits;

    #[test]
    pub fn test_parse() {
        let compiler = Compiler::acquire().unwrap();

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
        let input = ShaderInput::new(
            &source,
            &limits,
            ShaderStage::Fragment,
            &CompilerOptions::default(),
            None
        );
        let shader = Shader::new(&compiler, input).expect("shader init");

        let code = shader.get_preprocessed_code();

        println!("{}", code);
    }
}
