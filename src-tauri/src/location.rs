use std::io;
use std::path::{Path, PathBuf};

use serde::Serialize;
use walkdir::{WalkDir};

use crate::error::ApplicationError;

#[derive(Serialize)]
pub struct Location {
    root: PathBuf,
    paths: Vec<PathBuf>,
}

pub fn add_location(root: String) -> Result<Location, ApplicationError> {
    let root = Path::new(&root).to_path_buf();
    match read_dir(&root) {
        Ok(paths) => Ok(Location {
            root,
            paths,
        }),
        Err(e) => Err(ApplicationError::IOError(root, e.to_string())),
    }
}

fn read_dir(root: &PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    println!("Parsing directory: {}", root.display());

    if !root.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("{} is not a directory", root.display()),
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