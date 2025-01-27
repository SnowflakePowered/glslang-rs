use crate::ctypes::ShaderStage;
use crate::error::{GlslangError, GlslangErrorLog};
use crate::{Compiler, Shader};
use glslang_sys as sys;
use glslang_sys::glslang_spv_options_s;
use rustc_hash::FxHashMap;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::ptr::NonNull;

/// Lower-level program interface.
pub struct Program<'a> {
    handle: NonNull<sys::glslang_program_t>,
    cache: FxHashMap<ShaderStage, bool>,
    _compiler: PhantomData<&'a Compiler>,
}

impl<'a> Program<'a> {
    /// Create a new program instance.
    pub fn new(_compiler: &'a Compiler) -> Self {
        let program = Self {
            handle: unsafe {
                NonNull::new(sys::glslang_program_create()).expect("glslang created null shader")
            },
            cache: FxHashMap::default(),
            _compiler: PhantomData,
        };

        program
    }

    /// Add a shader to the program. The lifetime of the shader must extend beyond the lifetime of the program instance.
    pub fn add_shader<'shader>(&mut self, shader: &'shader Shader<'shader>)
    where
        'shader: 'a,
    {
        unsafe { sys::glslang_program_add_shader(self.handle.as_ptr(), shader.handle.as_ptr()) }
        self.cache.insert(shader.stage, shader.is_spirv);
    }

    /// Map shader input/output locations. Requires [crate::ShaderOptions::AUTO_MAP_LOCATIONS] to be set
    /// on shaders.
    pub fn map_io(&mut self) -> Result<(), GlslangError> {
        if unsafe { sys::glslang_program_map_io(self.handle.as_ptr()) } == 0 {
            return Err(GlslangError::MapIoError(GlslangErrorLog::new(self.get_log(), self.get_debug_log())));
        }

        Ok(())
    }

    /// Link the program without compiling to SPIR-V.
    ///
    /// A [`Program`](crate::Program) can only be linked once.
    pub fn link(self) -> Result<(), GlslangError> {
        let messages = glslang_sys::glslang_messages_t::DEFAULT
            | glslang_sys::glslang_messages_t::VULKAN_RULES
            | glslang_sys::glslang_messages_t::SPV_RULES;

        if unsafe { sys::glslang_program_link(self.handle.as_ptr(), messages.0) } == 0 {
            return Err(GlslangError::LinkError(GlslangErrorLog::new(self.get_log(), self.get_debug_log())));
        }
        Ok(())
    }

    /// Compile the given stage to SPIR-V, consuming the program.
    ///
    /// A [`Program`](crate::Program) can not be re-used to compile multiple stages.
    pub fn compile(self, stage: ShaderStage) -> Result<Vec<u32>, GlslangError> {
        // If the stage was not previously added to the program, compiling SPIRV ends up segfaulting.
        if !self.cache.contains_key(&stage) {
            return Err(GlslangError::ShaderStageNotFound(stage));
        }

        if let Some(false) = self.cache.get(&stage) {
            return Err(GlslangError::NoLanguageTarget);
        }

        let messages = glslang_sys::glslang_messages_t::DEFAULT
            | glslang_sys::glslang_messages_t::VULKAN_RULES
            | glslang_sys::glslang_messages_t::SPV_RULES;

        if unsafe { sys::glslang_program_link(self.handle.as_ptr(), messages.0) } == 0 {
            return Err(GlslangError::LinkError(GlslangErrorLog::new(self.get_log(), self.get_debug_log())));
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

    /// Compile the given stage to SPIR-V, optimizing for size, consuming the program.
    ///
    /// A [`Program`](crate::Program) can not be re-used to compile multiple stages.
    pub fn compile_size_optimized(self, stage: ShaderStage) -> Result<Vec<u32>, GlslangError> {
        // If the stage was not previously added to the program, compiling SPIRV ends up segfaulting.
        if !self.cache.contains_key(&stage) {
            return Err(GlslangError::ShaderStageNotFound(stage));
        }

        if let Some(false) = self.cache.get(&stage) {
            return Err(GlslangError::NoLanguageTarget);
        }

        let messages = glslang_sys::glslang_messages_t::DEFAULT
            | glslang_sys::glslang_messages_t::VULKAN_RULES
            | glslang_sys::glslang_messages_t::SPV_RULES;

        if unsafe { sys::glslang_program_link(self.handle.as_ptr(), messages.0) } == 0 {
            return Err(GlslangError::LinkError(GlslangErrorLog::new(self.get_log(), self.get_debug_log())));
        }

        let mut options = glslang_spv_options_s {
            generate_debug_info: false,
            strip_debug_info: false,
            disable_optimizer: false,
            optimize_size: true,
            disassemble: false,
            validate: false,
            emit_nonsemantic_shader_debug_info: false,
            emit_nonsemantic_shader_debug_source: false,
            compile_only: false,
            optimize_allow_expanded_id_bound: false,
        };

        // We don't support SPIRV compile options because nearly all of them (except for generateDebugInfo),
        // require callbacks that either we don't expose, or are not exposed by the C API.
        // disableOptimizer is redundant as well because we need to support WASM, which doesn't support
        // the optimizer.
        unsafe {
            sys::glslang_program_SPIRV_generate_with_options(
                self.handle.as_ptr(),
                stage,
                &mut options,
            )
        }

        let size = unsafe { sys::glslang_program_SPIRV_get_size(self.handle.as_ptr()) };
        let mut buffer = vec![0u32; size];

        unsafe {
            sys::glslang_program_SPIRV_get(self.handle.as_ptr(), buffer.as_mut_ptr());
        }

        Ok(buffer)
    }

    pub fn get_log(&self) -> String {
        let c_str =
            unsafe { CStr::from_ptr(sys::glslang_program_get_info_log(self.handle.as_ptr())) };

        let string = CString::from(c_str)
            .into_string()
            .expect("Expected glslang info log to be valid UTF-8");

        string
    }

    pub fn get_debug_log(&self) -> String {
        let c_str =
            unsafe { CStr::from_ptr(sys::glslang_program_get_info_debug_log(self.handle.as_ptr())) };

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
    use crate::include::{IncludeHandler, IncludeResult};
    use crate::shader::{CompilerOptions, OpenGlVersion, ShaderInput, ShaderSource, Target};
    use crate::{GlslProfile, ShaderMessage, SourceLanguage};
    use rspirv::binary::Disassemble;

    #[test]
    pub fn test_link() {
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

        let input = ShaderInput::new(
            &source,
            ShaderStage::Fragment,
            &CompilerOptions::default(),
            None,
            None,
        )
        .expect("target");
        let _shader = Shader::new(&compiler, input).expect("shader init");

        let program = Program::new(&compiler);
        // program.add_shader(&shader);

        program.link().expect("shader");
    }

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

        let input = ShaderInput::new(
            &source,
            ShaderStage::Fragment,
            &CompilerOptions::default(),
            None,
            None,
        )
        .expect("target");
        let shader = Shader::new(&compiler, input).expect("shader init");
        let code = shader.compile().expect("compile");
        let mut loader = rspirv::dr::Loader::new();
        rspirv::binary::parse_words(&code, &mut loader).unwrap();
        let module = loader.module();

        println!("{}", module.disassemble())
    }

    #[test]
    pub fn test_compile_thread() {
        let mut handles = Vec::new();
        for _ in 0..8 {
            handles.push(std::thread::spawn(|| test_compile()));
        }

        for handle in handles {
            handle.join().unwrap()
        }
    }

    #[test]
    pub fn test_verify_old_gl() {
        let compiler = Compiler::acquire().unwrap();

        let source = ShaderSource::from(String::from(
            r#"#version 120

varying vec2 texcoord;

void main() {
    gl_Position = ftransform();
    texcoord = gl_MultiTexCoord0.st;
}
        "#,
        ));

        let input = ShaderInput::new(
            &source,
            ShaderStage::Vertex,
            &CompilerOptions {
                source_language: SourceLanguage::GLSL,
                target: Target::OpenGL {
                    version: OpenGlVersion::OpenGL4_5,
                    spirv_version: None,
                },
                messages: ShaderMessage::DEBUG_INFO | ShaderMessage::DEFAULT,
                version_profile: Some((120, GlslProfile::None)),
            },
            None,
            None,
        )
        .expect("target");
        let _shader = Shader::new(&compiler, input).expect("shader init");
    }

    #[test]
    pub fn test_no_language_target_does_not_segfault() {
        let compiler = Compiler::acquire().unwrap();

        let source = ShaderSource::try_from(String::from(
            r#"
#version 460


layout(location = 0) out vec4 color;
layout(binding = 1) uniform sampler2D tex;

void main() {
    color = texture(tex, vec2(0.0));
}
        "#,
        ))
        .expect("source");

        let input = ShaderInput::new(
            &source,
            ShaderStage::Vertex,
            &CompilerOptions {
                source_language: SourceLanguage::GLSL,
                target: Target::None(None),
                messages: ShaderMessage::DEBUG_INFO | ShaderMessage::DEFAULT,
                version_profile: None,
            },
            None,
            None,
        )
        .expect("target");
        let shader = Shader::new(&compiler, input).expect("shader init");
        assert!(matches!(
            shader.compile(),
            Err(GlslangError::NoLanguageTarget)
        ));
    }

    #[test]
    pub fn test_compile_program() {
        let compiler = Compiler::acquire().unwrap();

        let fragment = ShaderSource::from(
            r#"
#version 450

layout(location = 0) out vec4 color;
layout(binding = 1) uniform sampler2D tex;

void main() {
    color = texture(tex, vec2(0.0));
}
        "#,
        );

        let vertex = ShaderSource::from(
            r#"
#version 450
layout(set = 0, binding = 0, std140) uniform UBO
{
    mat4 MVP;
};

layout(location = 0) in vec4 Position;
layout(location = 1) in vec2 TexCoord;
layout(location = 0) out vec2 vTexCoord;
void main()
{
    gl_Position = MVP * Position;
    vTexCoord = TexCoord;
}
"#,
        );

        let fragment = ShaderInput::new(
            &fragment,
            ShaderStage::Fragment,
            &CompilerOptions::default(),
            None,
            None,
        )
        .expect("target");
        let fragment = Shader::new(&compiler, fragment).expect("shader init");

        let vertex = ShaderInput::new(
            &vertex,
            ShaderStage::Vertex,
            &CompilerOptions::default(),
            None,
            None,
        )
        .expect("target");
        let vertex = Shader::new(&compiler, vertex).expect("shader init");

        let mut program = Program::new(&compiler);

        program.add_shader(&fragment);
        program.add_shader(&vertex);

        let _code = program.compile(ShaderStage::Fragment).expect("shader");

        let mut program = compiler.create_program();
        program.add_shader(&vertex);
        let code2 = program.compile(ShaderStage::Vertex).expect("shader");

        let mut loader = rspirv::dr::Loader::new();
        rspirv::binary::parse_words(&code2, &mut loader).unwrap();
        let module = loader.module();

        println!("{}", module.disassemble());
    }

    #[test]
    pub fn test_add_macros() {
        let compiler = Compiler::acquire().unwrap();

        let source = ShaderSource::try_from(String::from(
            r#"
#version 460

layout(location = 0) out vec4 color;

void main() {
    color = vec4(CUSTOM_MACRO);
}
        "#,
        ))
        .expect("source");

        let input = ShaderInput::new(
            &source,
            ShaderStage::Vertex,
            &CompilerOptions {
                source_language: SourceLanguage::GLSL,
                target: Target::None(None),
                messages: ShaderMessage::DEBUG_INFO | ShaderMessage::DEFAULT,
                version_profile: None,
            },
            Some(&[("CUSTOM_MACRO", Some("1.0"))]),
            None,
        )
        .expect("target");
        let _shader = Shader::new(&compiler, input).expect("shader init");
    }

    #[test]
    pub fn test_include_handler() {
        let compiler = Compiler::acquire().unwrap();

        struct MyIncludeHandler {
            header_included: Vec<String>,
        }
        impl IncludeHandler for MyIncludeHandler {
            fn include(
                &mut self,
                _ty: crate::include::IncludeType,
                header_name: &str,
                _includer_name: &str,
                _include_depth: usize,
            ) -> Option<IncludeResult> {
                self.header_included.push(header_name.into());
                Some(IncludeResult {
                    name: "included_macro".into(),
                    data: "#define INCLUDED_MACRO 0.0".into(),
                })
            }
        }

        let source = ShaderSource::try_from(String::from(
            r#"
#version 460
#extension GL_GOOGLE_include_directive : require
#include "custom_include.glsl"

layout(location = 0) out vec4 color;

void main() {
    color = vec4(INCLUDED_MACRO);
}
        "#,
        ))
        .expect("source");
        let mut include_handler = MyIncludeHandler {
            header_included: vec![],
        };
        let input = ShaderInput::new(
            &source,
            ShaderStage::Vertex,
            &CompilerOptions {
                source_language: SourceLanguage::GLSL,
                target: Target::OpenGL {
                    version: OpenGlVersion::OpenGL4_5,
                    spirv_version: None,
                },
                messages: ShaderMessage::DEBUG_INFO | ShaderMessage::DEFAULT,
                version_profile: None,
            },
            Some(&[("CUSTOM_MACRO", Some("1.0"))]),
            Some(&mut include_handler),
        )
        .expect("target");
        let _shader = Shader::new(&compiler, input).expect("shader init");
        assert!(include_handler.header_included.len() == 1);
        assert_eq!(
            include_handler.header_included[0], "custom_include.glsl",
            ""
        );
    }
}
