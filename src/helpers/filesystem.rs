use glob::{glob, GlobError};
use std::{fs, fs::File, path::Path, path::PathBuf, io::prelude::Write};

/// **Description**:
///
/// Checks if a given directory exists, shorthand for `Path::new(path).exists()`
///
/// **References**:
/// * [Path::new(...).exists()](https://doc.rust-lang.org/stable/std/path/struct.Path.html#method.exists)
pub fn exists(path: &str) -> bool {
  Path::new(path).exists()
}

/// **Description**:
///
/// Read content from a file
///
/// **Behaviour**:
/// * Open the file in read only mode
/// * Read and return it's contents
/// * Exit on success, panic on fail
///
/// **References**:
/// * [fs::read_to_string](https://doc.rust-lang.org/std/fs/fn.read_to_string.html)
pub fn read_file(file_path: &str) -> String {
  fs::read_to_string(file_path).expect(&format!("Error: failed to read file: \"{}\"", file_path))
}

/// **Description**:
///
/// Write a content string to a file
///
/// **Behaviour**:
/// * Open the file in write only mode
/// * Create it if it doesn't exist and truncate if it does
/// * Exit on success, panic on fail
///
/// **References**:
/// * [io::Write](https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all)
pub fn write_file(path: &str, content: String) {
  // Ensure the file's directory exists
  self::ensure_dir(&path[0..path.rfind('/').unwrap()]);
  // Create the write error message
  let write_error: &str = &format!("Error: failed to write file \"{}\"", &path);
  // Opens the file in write-only mode
  // Create the file if it doesn't exist / truncate it if it does
  let mut file: File = File::create(path).expect(write_error);
  // Write to the file
  file.write_all(content.as_bytes()).expect(write_error);
}

/// **Description**:
///
/// Copy a file from a source to a destination, with explicit overwrite permissions
///
/// **Behaviour**:
/// * Ensure the destination directory exists using `lib::ensure`
/// * Only write to the file if it doesn't already exist or if `overwrite` is `true`
/// * Exit on success, panic on fail
///
/// **References**:
/// * [fs::copy](https://doc.rust-lang.org/std/fs/fn.copy.html)
pub fn copy_file(from: &str, to: &str, overwrite: bool) {
  // Ensure the directory of the copy destination exists
  self::ensure_dir(&to[0..to.rfind('/').unwrap()]);
  // Write to the file if it doesn't exist or if overwrite is enabled
  if !self::exists(to) || overwrite == true {
    match fs::copy(from, to) {
      Ok(_) => { /* do nothing on success */ },
      Err(error) => panic!("Error: failed copy of \"{}\" to \"{}\"\n\n{}", from, to, error)
    }
  }
}

/// **Description**:
///
/// Checks if a given directory exists and creates it if it doesn't
///
/// **References**:
/// * [Path::new(...).exists()](https://doc.rust-lang.org/stable/std/path/struct.Path.html#method.exists)
/// * [fs::create_dir](https://doc.rust-lang.org/std/fs/fn.create_dir.html)
pub fn ensure_dir(path: &str) {
  if !self::exists(path) {
    match fs::create_dir_all(path) {
      Ok(_) => { /* IGNORE */ },
      Err(error) => panic!("Error: failed to create directory \"{}\"\n\n{}", path, error)
    }
  }
}

/// **Description**:
///
/// Empties a specified directory
///
/// **Behaviour**:
/// * Avoid not exists errors (`lib::exists`)
///   * If the directory doesn't exist, it's created and the function exits
/// * Deletes the directory after deleting it's contents (`fs::remove_dir_all`)
/// * Create a new directory at the given path (`fs::create_dir`)
///
/// **Notes**:
/// * Fails if user lacks permissions for the directory
/// * Does not follow symbolic links it simply removes the link itself
///
/// **References**:
/// * [fs::remove_dir_all](https://doc.rust-lang.org/std/fs/fn.remove_dir_all.html)
/// * [fs::create_dir](https://doc.rust-lang.org/std/fs/fn.create_dir.html)
/// * [Path::new(...).exists()](https://doc.rust-lang.org/stable/std/path/struct.Path.html#method.exists)
pub fn empty_dir(dir_path: &str) {
  let empty_error: &str = &format!("Error: failed to empty dir \"{}\"", dir_path);
  if self::exists(dir_path) {
    fs::remove_dir_all(dir_path).expect(empty_error);
  }
  fs::create_dir(dir_path).expect(empty_error);
}

/// **Description**:
///
/// Glob a directory and return a Vector of Strings (wrapper for [glob::glob](https://docs.rs/glob/0.3.0/glob/fn.glob.html))
///
/// Where:
///
/// `lib::walk("./")`
///
/// Returns a Vec\<String\> containing the paths of each item in the current directory
///
/// Info: name inspired by [os.walk](https://docs.python.org/3/library/os.html#os.walk) in Python
pub fn walk_dir(directory: &str) -> Vec<String> {
  // Provide more information on panic
  let glob_error: &str = &format!("Error: failed to glob directory \"{}\"", directory);
  // Glob the given directory
  glob(directory)
    // Show the glob error on panic
    .expect(glob_error)
    // Convert each PathBuf into a String
    .map(| path: Result<PathBuf, GlobError> | path.expect(glob_error).display().to_string())
    // Collect the path String's into a Vector
    .collect::<Vec<String>>()
}