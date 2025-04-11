use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum UtilFuncError {
    #[error("Failed to create directory '{path}': '{source}'")]
    CreateFileError{
        path: String,
        source: io::Error,

    },
    #[error("Failed to create file '{path}': '{source}'")]
    CreateDirError{
        path: String,
        source: io::Error,
    },
    #[error("Failed to write to file '{path}': '{source}'")]
    WriteFileError{
        path: String,
        source: io::Error,
    },
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("toml")]
    TomlError(#[from] toml::ser::Error)
}

#[derive(Error, Debug)]
pub enum VenvError {
    #[error("venv check failed: {source}")]
    VenvCheckFailed { source: std::io::Error },
    #[error("venv creation failed: {source}")]
    VenvCreationFailed { source: std::io::Error },
    #[error("conda check failed: {source}")]
    CondaCheckFailed { source: std::io::Error },
    #[error("conda creation failed: {source}")]
    CondaCreationFailed { source: std::io::Error },
}