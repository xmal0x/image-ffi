use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageError {
    #[error("File not found")]
    FileNotFound,
    #[error("Decode error: {0}")]
    DecodeError(String),
    #[error("Read params error: {0}")]
    ReadParamsError(String),
    #[error("Plugin error: {0}")]
    PluginError(String),
    #[error("Invalid buffer size")]
    InvalidBufferSize,
    #[error("Save image error: {0}")]
    SaveImageError(String),
}
