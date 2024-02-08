use std::sync::OnceLock;

mod ctypes;
mod shader;

pub mod error;
pub mod input;
pub mod limits;
mod program;
mod callbacks;

static COMPILER_INSTANCE: OnceLock<Option<Compiler>> = OnceLock::new();
pub struct Compiler;

pub use crate::ctypes::*;

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
}

impl Drop for Compiler {
    fn drop(&mut self) {
        unsafe {
            glslang_sys::glslang_finalize_process();
        }
    }
}
