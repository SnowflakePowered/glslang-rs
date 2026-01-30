use crate::GlslProfile;
use crate::ctypes::ShaderStage;
use crate::shader::Target;
use thiserror::Error;

/// The error type for `glslang`.
#[derive(Error, Debug, Clone, Hash, PartialEq, Eq)]
pub enum GlslangError {
    /// Error occurred when preprocessing.
    #[error("preprocess error: {0}")]
    PreprocessError(String),
    /// Error occurred when preprocessing.
    #[error("parse error: {0}")]
    ParseError(String),
    /// Error occurred when mapping IO.
    #[error("map io error: {0}")]
    MapIoError(String),
    /// Error occurred when linking
    #[error("program link error: {0}")]
    LinkError(String),
    /// The shader stage was not found in the program.
    #[error("shader stage not found: {0:?}")]
    ShaderStageNotFound(ShaderStage),
    /// No SPIR-V language target was set.
    #[error("tried to compile shader with no language target")]
    NoLanguageTarget,
    /// The target is not a valid combination of environment, version, and language version.
    #[error("the target is invalid")]
    InvalidTarget(Target),
    /// The GLSL profile and version is not valid for the specified target.
    #[error("the profile is invalid")]
    InvalidProfile(Target, i32, GlslProfile),
    /// The GLSL version is unsupported for the profile
    #[error("the profile is invalid")]
    VersionUnsupported(i32, GlslProfile),
}
