use std::sync::OnceLock;

mod ctypes;
mod shader;

pub mod error;
pub mod include;
pub mod input;
pub mod limits;
mod program;

static COMPILER_INSTANCE: OnceLock<Option<Compiler>> = OnceLock::new();
pub struct Compiler;

pub use crate::ctypes::*;

use crate::error::GlslangError;
use crate::input::ShaderInput;
pub use program::Program;
pub use shader::Shader;

impl Compiler {
    pub fn acquire() -> Option<&'static Self> {
        COMPILER_INSTANCE
            .get_or_init(|| {
                unsafe {
                    if glslang_sys::glslang_initialize_process() == 0 {
                        return None;
                    }
                };
                Some(Self)
            })
            .as_ref()
    }

    pub fn create_shader(&self, input: ShaderInput) -> Result<Shader, GlslangError> {
        Shader::new(&self, input)
    }

    pub fn create_program(&self) -> Program {
        Program::new(&self)
    }
}

impl Drop for Compiler {
    fn drop(&mut self) {
        unsafe {
            glslang_sys::glslang_finalize_process();
        }
    }
}
