use crate::ctypes::ShaderStage;
use std::ffi::NulError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GlslangError {
    /// Error occurred when preprocessing.
    #[error("preprocess error: {0}")]
    PreprocessError(String),
    /// Error occurred when preprocessing.
    #[error("parse error: {0}")]
    ParseError(String),
    #[error("map io error: {0}")]
    MapIoError(String),
    #[error("program link error: {0}")]
    LinkError(String),
    #[error("null error")]
    NulError(#[from] NulError),
    #[error("shader stage not found: {0:?}")]
    ShaderStageNotFound(ShaderStage),
}
