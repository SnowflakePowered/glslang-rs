use crate::ctypes::ShaderStage;
use crate::error::GlslangError;
use crate::input::ShaderInput;
use crate::{Compiler, Shader};
use glslang_sys as sys;
use rustc_hash::FxHashSet;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ptr::NonNull;

pub struct Program<'a> {
    handle: NonNull<sys::glslang_program_t>,
    cache: FxHashSet<ShaderStage>,
    _compiler: PhantomData<&'a Compiler>,
}

impl<'a> Program<'a> {
    pub fn new(_compiler: &'a Compiler) -> Self {
        let program = Self {
            handle: unsafe {
                NonNull::new(sys::glslang_program_create()).expect("glslang created null shader")
            },
            cache: FxHashSet::default(),
            _compiler: PhantomData,
        };

        program
    }

    pub fn add_shader(&mut self, shader: &Shader<'_>) {
        unsafe { sys::glslang_program_add_shader(self.handle.as_ptr(), shader.handle.as_ptr()) }
        self.cache.insert(shader.stage);
    }

    /// Map shader input/output locations. Requires `GLSLANG_SHADER_AUTO_MAP_LOCATIONS`
    pub fn map_io(&mut self) -> Result<(), GlslangError> {
        if unsafe { sys::glslang_program_map_io(self.handle.as_ptr()) } == 0 {
            return Err(GlslangError::MapIoError(self.get_log()));
        }

        Ok(())
    }

    pub fn link(&mut self) -> Result<(), GlslangError> {
        let messages = glslang_sys::glslang_messages_t::GLSLANG_MSG_DEFAULT_BIT
            | glslang_sys::glslang_messages_t::GLSLANG_MSG_VULKAN_RULES_BIT
            | glslang_sys::glslang_messages_t::GLSLANG_MSG_SPV_RULES_BIT;

        if unsafe { sys::glslang_program_link(self.handle.as_ptr(), messages.0) } == 0 {
            return Err(GlslangError::LinkError(self.get_log()));
        }

        Ok(())
    }

    pub fn compile(&mut self, stage: ShaderStage) -> Result<Vec<u32>, GlslangError> {
        // If the stage was not previously added to the program, compiling SPIRV ends up segfaulting.
        if !self.cache.contains(&stage) {
            return Err(GlslangError::ShaderStageNotFound(stage))
        }

        // We don't support SPIRV compile options because nearly all of them (except for generateDebugInfo),
        // require callbacks that either we don't expose, or are not exposed by the C API.
        // disableOptimizer is redundant as well because we need to support WASM, which doesn't support
        // the optimizer.
        unsafe { sys::glslang_program_SPIRV_generate(self.handle.as_ptr(), stage) }

        let size = unsafe { sys::glslang_program_SPIRV_get_size(self.handle.as_ptr()) };
        let mut buffer = vec![0u32; size];

        unsafe {
            sys::glslang_program_SPIRV_get(self.handle.as_ptr(), buffer.as_mut_ptr());
        }

        Ok(buffer)
    }
    fn get_log(&self) -> String {
        let c_str =
            unsafe { CStr::from_ptr(sys::glslang_program_get_info_log(self.handle.as_ptr())) };

        let string = CString::from(c_str)
            .into_string()
            .expect("Expected glslang info log to be valid UTF-8");

        string
    }
}

impl<'a> Drop for Program<'a> {
    fn drop(&mut self) {
        unsafe { sys::glslang_program_delete(self.handle.as_ptr()) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ctypes::ShaderStage;
    use crate::input::ShaderSource;
    use crate::limits::ResourceLimits;
    use rspirv::binary::Disassemble;
    #[test]
    pub fn test_compile() {
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
        let input = ShaderInput::new(&source, &limits, ShaderStage::GLSLANG_STAGE_FRAGMENT);
        let shader = Shader::new(&compiler, input).expect("shader init");

        let mut program = Program::new(&compiler);

        program.add_shader(&shader);
        program.link().expect("link error");

        let code = program.compile(ShaderStage::GLSLANG_STAGE_FRAGMENT)
            .expect("shader");
        //
        let mut loader = rspirv::dr::Loader::new();
        rspirv::binary::parse_words(&code, &mut loader).unwrap();
        let module = loader.module();

        println!("{}", module.disassemble())
    }
}
