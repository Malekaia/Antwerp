//! **Description**:
//!
//! Standardises and simplifies file/folder path handling
use std::{env, path::{Path, PathBuf}};

/// **Description**:
///
/// Joins an absolute parent path with a child path
pub fn join(parent: &str, child: &str) -> String {
  // Create the path
  Path::new(parent)
      // Get the absolute form of the path
      .canonicalize()
      // Handle errors
      .expect(&format!("Error: failed to join \"{}\" with \"{}\"", parent, child))
      // Add the child path to the parent path
      .join(child)
      // Convert the path into a string
      .display().to_string()
      // Remove unnecessary relative paths
      .replace("/./", "/")
}

/// **Description**:
///
/// Converts a path into its absolute format using the CWD as its root
pub fn from_cwd(child: &str) -> String {
  // Get the current working directory
  let current_working_directory: PathBuf = env::current_dir().expect("Error: failed to get CWD");
  // Create the path
  Path::new(&current_working_directory)
      // Get the absolute form of the CWD
      .canonicalize()
      // Handle errors
      .expect(&format!("Error: failed to create absolute path from CWD"))
      // Add the child path to the parent path
      .join(child)
      // Convert the path into a string
      .display().to_string()
      // Remove unnecessary relative paths
      .replace("/./", "/")
}

/// **Description**:
///
/// Converts a given path to it's absolute verison
pub fn absolute(child: &str) -> String {
  // Create the path
  Path::new(child)
      // Get the absolute form of the child path
      .canonicalize()
      // Handle errors
      .expect(&format!("Error: "))
      // Convert the path into a string
      .display().to_string()
      // Remove unnecessary relative paths
      .replace("/./", "/")
}