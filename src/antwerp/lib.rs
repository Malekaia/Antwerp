//! Exposes functions that prevent code duplication - and standardises the implementation of frequently used tools in a fail-safe way, providing detailed error handling if not possible.
use colored::Colorize;
use glob::{glob, GlobError};
use regex::Regex;
use std::{fs, fs::File, path::Path, path::PathBuf, io::prelude::Write};

/// **Description**:
///
/// Use `println!` to output a colourised string about a certain action
pub fn log(verbose: bool, color: &str, action: &str, category: &str, target: &str) {
  if verbose == true {
    println!("{} ({}): {}", format!("{action}").color(color).bold(), format!("{category}").bold(), target);
  }
}

/// **Description**:
///
/// Standardises and simplifies file/folder path handling
pub mod path {
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
         .expect(&*format!("Error: failed to join \"{}\" with \"{}\"", parent, child))
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
         .expect(&*format!("Error: failed to create absolute path from CWD"))
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
         .expect(&*format!("Error: "))
         // Convert the path into a string
         .display().to_string()
         // Remove unnecessary relative paths
         .replace("/./", "/")
  }
}

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
  let empty_error: &str = &format!("Failed to empty dir \"{}\"", dir_path);
  if !self::exists(dir_path) {
    fs::create_dir(dir_path).expect(empty_error);
  } else {
    fs::remove_dir_all(dir_path).expect(empty_error);
    fs::create_dir(dir_path).expect(empty_error);
  }
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

/// **Description**:
///
/// Convert a string to slug case
///
/// **References**:
/// * [What is a slug?](https://stackoverflow.com/a/19335586/10415695)
pub fn string_to_slug(value: &String) -> String {
  // remove all non alpha numeric values
  let re_non_alpha_num: Regex = Regex::new(r"[^a-zA-Z0-9]+").unwrap();
  let result = re_non_alpha_num.replace_all(&value.to_lowercase(), " ").to_string();
  // replace all spaces with hyphens and trim
  let re_delimiter: Regex = Regex::new(r"\s+").unwrap();
  re_delimiter.replace_all(result.trim(), "-").to_string()
}

/// A list of characters and their corresponding HTML entities
static HTML_ESCAPABLE: [[&str; 2]; 31] = [
  [r"!", "&#33;"], // Exclamation mark
  ["\"", "&#34;"], // Quotation mark
  [r"#", "&#35;"], // Number sign
  [r"$", "&#36;"], // Dollar sign
  [r"%", "&#37;"], // Percent sign
  [r"&", "&#38;"], // Ampersand
  [r"'", "&#39;"], // Apostrophe
  [r"(", "&#40;"], // Opening/Left Parenthesis
  [r")", "&#41;"], // Closing/Right Parenthesis
  [r"*", "&#42;"], // Asterisk
  [r"+", "&#43;"], // Plus sign
  [r",", "&#44;"], // Comma
  [r"-", "&#45;"], // Hyphen
  [r".", "&#46;"], // Period
  [r":", "&#58;"], // Colon
  [r";", "&#59;"], // Semicolon
  [r"<", "&#60;"], // Less-than
  [r"=", "&#61;"], // Equals sign
  [r">", "&#62;"], // Greater than
  [r"?", "&#63;"], // Question mark
  [r"@", "&#64;"], // At sign
  [r"[", "&#91;"], // Opening/Left square bracket
  [r"\", "&#92;"], // Backslash
  [r"]", "&#93;"], // Closing/Right square bracket
  [r"^", "&#94;"], // Caret
  [r"_", "&#95;"], // Underscore
  [r"`", "&#96;"], // Grave accent
  ["{", "&#123;"], // Opening/Left curly brace
  ["|", "&#124;"], // Vertical bar
  ["}", "&#125;"], // Closing/Right curly brace
  ["~", "&#126;"]  // Tilde
];

/// **Description**:
///
/// Escapes and sanitizes a string of HTML using `lib::HTML_ESCAPABLE` to provide a list of values to escape
///
/// **References**:
/// * [HTML Sanitization](https://en.wikipedia.org/wiki/HTML_sanitization)
/// * [HTML Entity List](https://www.freeformatter.com/html-entities.html#misc-html-entities)
pub fn escape_html(html: &String) -> String {
  // Replace the characters with their matching HTML entities
  let mut result: String = html.to_owned();
  for [character, replacement] in HTML_ESCAPABLE {
    result = result.replace(character, replacement);
  }
  result
}
