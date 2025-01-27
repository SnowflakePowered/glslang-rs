use crate::ctypes::ShaderStage;
use crate::shader::Target;
use crate::GlslProfile;
use thiserror::Error;

/// The error logs
#[derive(Debug)]
pub struct GlslangErrorLog {
    pub log: String,
    pub debug_log: String,
}

impl GlslangErrorLog {
    pub fn new(log: String, debug_log: String) -> Self {
        Self {
            log, debug_log
        }
    }
}

impl std::fmt::Display for GlslangErrorLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Log:\n{}\nDebugLog:\n{}", self.log, self.debug_log)
    }
}

/// The error type for `glslang`.
#[derive(Debug, Error)]
pub enum GlslangError {
    /// Error occurred when preprocessing.
    #[error("preprocess error: {0}")]
    PreprocessError(GlslangErrorLog),
    /// Error occurred when preprocessing.
    #[error("parse error: {0}")]
    ParseError(GlslangErrorLog),
    /// Error occurred when mapping IO.
    #[error("map io error: {0}")]
    MapIoError(GlslangErrorLog),
    /// Error occurred when linking
    #[error("program link error: {0}")]
    LinkError(GlslangErrorLog),
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
