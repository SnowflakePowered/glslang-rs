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
    #[error("null error")]
    NulError(#[from] NulError),
}
