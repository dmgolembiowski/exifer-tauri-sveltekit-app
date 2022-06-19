use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use imagesize::ImageType;
use serde::Serialize;
use walkdir::WalkDir;

use crate::error::ApplicationError;
use crate::extension::{Extension, match_extension};

#[derive(Serialize)]
pub struct LocationStatistic {
    pub root: PathBuf,
    pub total_image_file_count: u32,
    pub total_video_file_count: u32,
    pub total_unknown_file_count: u32,
    pub average_image_file_size: u32,
    pub average_video_file_size: u32,
    pub largest_image_file_size: u32,
    pub largest_video_file_size: u32,
    pub largest_image_file: OsString,
    pub largest_video_file: OsString,
}

#[derive(Clone, Debug, Default)]
pub struct ImageDetails {
    pub path: PathBuf,
    pub title: OsString,
    pub width: u32,
    pub height: u32,
    pub advertised_extension: Extension,
    pub determined_extension: Extension,
    pub content_length: u64,
}

#[derive(Clone, Debug, Default)]
pub struct FsDetails {
    pub path: PathBuf,
    pub extension: Extension,
    pub content_length: u64,
}

pub fn generate_location_statistics(dir: String) -> Result<LocationStatistic, ApplicationError> {
    let dir_path = &PathBuf::from(dir);
    let paths = map_dir_to_paths(dir_path)?;

    let fs_details_list = paths
        .iter()
        .map(|path| parse_fs(path))
        .filter(|fs_details| fs_details.is_ok())
        .map(|fs_details| fs_details.unwrap())
        .collect::<Vec<FsDetails>>();

    let mut extension_map = fs_details_list.iter()
        .fold(HashMap::new(), |mut map, fs_details| {
            map.entry(fs_details.extension).or_insert(fs_details.clone());
            map
        });

    println!("{:?}", extension_map.values().count());

    let filtered_map = extension_map
        .drain_filter(|ext, _val| ext == &Extension::UNKNOWN)
        .map(|(_ext, val)| val)
        .collect::<Vec<FsDetails>>();

    println!("{:?}", filtered_map.len());
        // .collect::<Map<Extension, Vec<FsDetails>>>();

    Err(ApplicationError::IOError(dir_path.to_path_buf(), "hello".to_string()))
}

fn parse_fs(path: &PathBuf) -> Result<FsDetails, ApplicationError> {
    Ok(FsDetails {
        path: path.clone(),
        extension: parse_fs_extension(path)?,
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

fn confirm_extension(details: &ImageDetails) -> Extension {
    if (details.advertised_extension != details.determined_extension)
        || (details.advertised_extension == Extension::UNKNOWN)
        || (details.determined_extension == Extension::UNKNOWN) {
        return Extension::UNKNOWN;
    }
    details.determined_extension
}

fn parse_file(path: &Path) -> Result<ImageDetails, ApplicationError> {
    let (width, height) = parse_dimension_from_bytes(path)?;
    let extension = parse_file_type_from_bytes(path)?;

    Ok(ImageDetails {
        path: path.to_path_buf(),
        title: parse_fs_title(path)?,
        width,
        height,
        advertised_extension: parse_fs_extension(path)?,
        determined_extension: extension,
        content_length: parse_fs_content_length(path)?,
    })
}

fn map_dir_to_paths(dir: &PathBuf) -> Result<Vec<PathBuf>, ApplicationError> {
    if !dir.is_dir() {
        return Err(ApplicationError::IOError(dir.to_path_buf(),
                                             "Path is not a directory".to_string()));
    }

    let paths = WalkDir::new(dir)
        .max_depth(3)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|entry| entry.path().to_path_buf())
        .filter(|path| path.is_file())
        .collect::<Vec<PathBuf>>();

    Ok(paths)
}
