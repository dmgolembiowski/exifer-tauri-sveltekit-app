use std::path::PathBuf;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub enum ApplicationError {
    UnsupportedFileType(PathBuf, String),
    UnknownFileType(PathBuf, String),
    LibraryParseError(PathBuf, String),
    ExifParseError(PathBuf, String),
    NoFileName(PathBuf, String),
    IOError(PathBuf, String),
}

