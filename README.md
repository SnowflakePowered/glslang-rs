# glslang-rs

Safe Rust bindings to glslang

[![Latest Version](https://img.shields.io/crates/v/glslang.svg)](https://crates.io/crates/glslang) [![Docs](https://docs.rs/glslang/badge.svg)](https://docs.rs/glslang) ![License](https://img.shields.io/crates/l/glslang)

## Usage

```toml
[dependencies]
glslang = "0.3"
```

## Example
Compiling a shader

```rust
    use rspirv::binary::Disassemble;
    use glslang::*;

    #[test]
    pub fn test_compile() {
        // Acquire the compiler instance
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
            None,
        );
        let shader = Shader::new(&compiler, input).expect("shader init");

        let mut program = Program::new(&compiler);

        program.add_shader(shader);
        program.link().expect("link error");

        let code = program.compile(ShaderStage::Fragment).expect("shader");
       
        // Use rspirv to disassemble
        let mut loader = rspirv::dr::Loader::new();
        rspirv::binary::parse_words(&code, &mut loader).unwrap();
        let module = loader.module();

        println!("{}", module.disassemble())
    }
```