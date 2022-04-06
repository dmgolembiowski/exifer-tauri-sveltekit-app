use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use walkdir::WalkDir;

use crate::error::ApplicationError;
use crate::extension::Extension;
use crate::file_details;
use crate::file_details::FileDetails;

#[derive(Serialize, Deserialize)]
pub struct Location {
    root: PathBuf,
}

#[derive(Serialize)]
pub struct LocationDetails {
    pub total_files: u32,
    pub parsable_files: u32,
    pub largest_image_size: u32,
    pub extension_map: HashMap<Extension, u32>,
    pub errors: Vec<ApplicationError>,
}

pub fn add_location(root: String) -> Result<Location, ApplicationError> {
    let root = Path::new(&root).to_path_buf();
    println!("Adding directory {}", &root.display());

    if !root.is_dir() {
        return Err(ApplicationError::IOError(root, "Path is not a directory".to_string()));
    }

    Ok(Location { root })
}

fn read_dir(root: &PathBuf) -> Result<Vec<PathBuf>, ApplicationError> {
    println!("Parsing directory: {}", root.display());

    if !root.is_dir() {
        return Err(ApplicationError::IOError(
            root.clone(),
            "Path is not a directory".to_string(),
        ));
    }

    let paths = WalkDir::new(root)
        .max_depth(3)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|entry| entry.path().to_path_buf())
        .filter(|path| path.is_file())
        .collect::<Vec<PathBuf>>();

    Ok(paths)
}

pub fn analyse_location(location: Location) -> Result<LocationDetails, ApplicationError> {
    let paths = read_dir(&location.root)?;

    let mut details = file_details::parse_files(paths);
    println!("total :{}", details.len());

    let total_size = details.len();

    let errs = details.drain_filter(|result| {
        if result.is_err() {
            return true;
        }

        false
    })
        .map(|err| err.unwrap_err())
        .collect::<Vec<ApplicationError>>();

    println!("Errs :{}", errs.len());
    println!("Details :{}", details.len());

    let parsed_file_count = details.len();

    let file_details = details.into_iter()
        .filter(|fd| fd.is_ok())
        .map(|fd| fd.unwrap())
        .collect::<Vec<FileDetails>>();

    let max = file_details.iter().map(|fd| fd.content_length).max().unwrap_or(0);

    let map = file_details.iter()
        .fold(HashMap::new(),
              |mut acc, details| {
                  let key = confirm_extension(details);
                  acc.entry(key).or_insert(0);
                  acc.entry(key).and_modify(|e| *e += 1);
                  acc
              });

    Ok(LocationDetails {
        total_files: total_size as u32,
        parsable_files: parsed_file_count as u32,
        largest_image_size: max as u32,
        extension_map: map,
        errors: errs,
    })
}

fn confirm_extension(details: &FileDetails) -> Extension {
    if (details.advertised_extension != details.determined_extension)
        || (details.advertised_extension == Extension::UNKNOWN)
        || (details.determined_extension == Extension::UNKNOWN) {
        return Extension::UNKNOWN;
    }
    details.determined_extension
}