use std::sync::OnceLock;

mod ctypes;

/// Error types.
pub mod error;

/// Helpers for processing includes.
pub mod include;
/// Shader resouce limits.
pub mod limits;
mod program;
mod shader;

static COMPILER_INSTANCE: OnceLock<Option<Compiler>> = OnceLock::new();

/// A handle representing the glslang compiler instance.
#[derive(Debug)]
pub struct Compiler;

pub use crate::ctypes::*;
use crate::error::GlslangError;

pub use program::Program;
pub use shader::*;

impl Compiler {
    /// Acquire a global instance of the compiler.
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

    /// Create a [`Shader`](crate::Shader) with the given inputs.
    pub fn create_shader(&self, input: ShaderInput) -> Result<Shader<'_>, GlslangError> {
        Shader::new(&self, input).parse()
    }

    /// Create a [`Program`](crate::Program) instance.
    pub fn create_program(&self) -> Program<'_> {
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
