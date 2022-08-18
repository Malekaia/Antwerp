//! Standardises and simplifies file/folder path handling
use std::ffi::OsString;
use std::fs::{DirEntry, read_dir};
use std::path::{Ancestors, Path, PathBuf};
use std::{env, io};
use std::io::{Error, ErrorKind};

/// **Description**:
///
/// Helper to find the absolute root directory path of a project relative to the location of the nearest Cargo.lock file
fn get_project_root() -> io::Result<PathBuf> {
  let path: PathBuf = env::current_dir().expect("Error: failed to get current directory");
  let mut path_ancestors: Ancestors = path.as_path().ancestors();
  // Try to get the project root
  while let Some(p) = path_ancestors.next() {
    let has_cargo: bool = read_dir(p).expect("Error: failed to read directory for \"{p}\"").into_iter().any(| p: Result<DirEntry, Error> |
      p.unwrap().file_name() == OsString::from("Cargo.lock")
    );
    if has_cargo {
      return Ok(PathBuf::from(p))
    }
  }
  // Return an error if not possible
  Err(io::Error::new(ErrorKind::NotFound, "Error: could not find Cargo.toml"))
}

/// **Description**:
///
/// Helper to find the absolute path of a given string relative to the location of the nearest Cargo.lock file
pub fn path_absolute(file_path: &str) -> String {
  let root: &PathBuf = &get_project_root().expect("Error: failed to get absolute path for \"{file_path}\"");
  Path::new(root).join(file_path).into_os_string().into_string().unwrap()
}

/// **Description**:
///
/// Joins an absolute parent path with a child path
pub fn path_join(parent: &str, child: &str) -> String {
  Path::new(&path_absolute(parent)).join(child).display().to_string()
}