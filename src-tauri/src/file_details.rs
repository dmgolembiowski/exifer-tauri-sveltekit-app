use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use imagesize::ImageType;

use crate::error::ApplicationError;
use crate::extension::{Extension, match_extension};

#[derive(Clone, Debug, Default)]
pub struct FileDetails {
    pub path: PathBuf,
    pub title: OsString,
    pub width: u32,
    pub height: u32,
    pub advertised_extension: Extension,
    pub determined_extension: Extension,
    pub content_length: u64,
}

pub fn parse_files(paths: Vec<PathBuf>) -> Vec<Result<FileDetails, ApplicationError>> {
    paths.iter()
        .map(|path| parse_file(path))
        .collect()
}

pub fn parse_file(path: &Path) -> Result<FileDetails, ApplicationError> {
    let (width, height) = parse_dimension_from_bytes(path)?;
    let extension = parse_file_type_from_bytes(path)?;
    
    Ok(FileDetails {
            path: path.to_path_buf(),
            title: parse_fs_title(path)?,
            width,
            height,
            advertised_extension: parse_fs_extension(path)?,
            determined_extension: extension,
            content_length: parse_fs_content_length(path)?,
        })
}

fn parse_fs_title(path: &Path) -> Result<OsString, ApplicationError> {
    match path.file_name() {
        Some(value) => Ok(value.to_os_string()),
        None => {
            // Unsure if the name can actually not be present?
            Err(ApplicationError::NoFileName(PathBuf::from(path),
                                             "No file name detected".to_string()))
        }
    }
}

fn parse_fs_extension(path: &Path) -> Result<Extension, ApplicationError> {
    let io_ext = path.extension();

    if io_ext.is_none() {
        return Err(ApplicationError::UnknownFileType(PathBuf::from(path),
                                                     "Unknown file extension".to_string()));
    }

    let extension = match_extension(io_ext.unwrap());
    if extension != Extension::UNKNOWN {
        Ok(extension)
    } else {
        Err(ApplicationError::UnsupportedFileType(PathBuf::from(path),
                                                  "Unsupported file extension".to_string()))
    }
}

fn parse_fs_content_length(path: &Path) -> Result<u64, ApplicationError> {
    fs::metadata(path)
        .map(|m| m.len())
        .map_err(|e| ApplicationError::IOError(path.to_path_buf(), e.to_string()))
}

fn parse_dimension_from_bytes(path: &Path) -> Result<(u32, u32), ApplicationError> {
    match imagesize::size(path) {
        Ok(size) => Ok((size.width as u32, size.height as u32)),
        Err(e) => Err(ApplicationError::LibraryParseError(path.to_path_buf(), e.to_string()))
    }
}

fn parse_file_type_from_bytes(path: &Path) -> Result<Extension, ApplicationError> {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(&file);
    let mut header = [0; 12];

    reader.read_exact(&mut header).unwrap();

    match imagesize::image_type(header.as_ref()) {
        Ok(format) => {
            match format {
                ImageType::Jpeg => Ok(Extension::JPG),
                ImageType::Png => Ok(Extension::PNG),
                ImageType::Heif => Ok(Extension::HEIF),
                _ => Err(ApplicationError::UnsupportedFileType(path.to_path_buf(),
                                                               "Unsupported file type".to_string()))
            }
        }
        Err(e) => {
            Err(ApplicationError::LibraryParseError(path.to_path_buf(), e.to_string()))
        }
    }
}
