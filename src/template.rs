use crate::fileio;
use marcus;
use regex::Regex;
use std::{env::current_dir, ffi::OsString, fs::{read_dir, DirEntry}, collections::HashMap, io, path::{PathBuf, Ancestors}};

/// Stores block information
#[derive(Debug)]
pub struct Block {
  pub outer: String,
  pub inner: String
}

/// Stores template information
#[derive(Debug)]
pub struct Template {
  pub path_template: PathBuf,
  pub path_output: PathBuf,
  pub parent: String,
  pub blocks: Blocks
}

/// Shorthand for `HashMap<String, Block>`
pub type Blocks = HashMap<String, Block>;

/// Shorthand for `Vec<Template>`
pub type Templates = Vec<Template>;

// Define regular expressions once
lazy_static! {
  /// Regular expression for "extends" statements
  static ref RE_EXTENDS: Regex = Regex::new(r#"\{% extends "(.*?)" %\}"#).unwrap();
  /// Regular expression for "block" statements
  static ref RE_BLOCK: Regex = Regex::new(r#"\{% block (.*?) %\}"#).unwrap();
}

/// Find the absolute root directory path of a project as it stands relative to the location of the nearest Cargo.lock file
///
/// Crate: https://crates.io/crates/project-root
///
/// ```rust
/// match project_root::project_root() {
///     Ok(p) => println!("Current project root is {:?}", p),
///     Err(e) => println!("Error obtaining project root {:?}", e)
/// };
/// ```
fn project_root() -> io::Result<PathBuf> {
  let path: PathBuf = current_dir()?;
  let mut path_ancestors: Ancestors = path.as_path().ancestors();
  while let Some(p) = path_ancestors.next() {
    if read_dir(p)?.into_iter().any(| p: Result<DirEntry, io::Error> | p.unwrap().file_name() == OsString::from("Cargo.lock")) {
      return Ok(PathBuf::from(p))
    }
  }
  Err(io::Error::new(io::ErrorKind::NotFound, "Ran out of places to find Cargo.toml"))
}

/// Extract template data (parent template from extends, blocks and block contents)
fn extract_parent_blocks(content: &String) -> (String, Blocks) {
  // Ensure the template extends a parent template
  if !RE_EXTENDS.is_match(&content) {
    panic!("TemplateError: missing extend.");
  }
  // Create a store for the blocks
  let mut blocks: Blocks = HashMap::new();

  // Iterate all captures for "block" statements
  for capture in RE_BLOCK.captures_iter(&content) {
    // Get the block name
    let block_name: &str = &capture[1];
    // Create the "endblock" statement
    let end_block: &str = &format!("{{% endblock {} %}}", block_name);

    // Ensure all "block" statements have an accompanying "endblock" statement
    if !content.contains(end_block) {
      panic!("TemplateError: missing \"endblock\" for {}", block_name);
    }
    // Check for duplicate "block" statements
    else if content.matches(&capture[0]).count() > 1 {
      panic!("TemplateError: duplicate \"block\" for \"{}\"", block_name);
    }
    // Check for duplicate "endblock" statements
    else if content.matches(end_block).count() > 1 {
      panic!("TemplateError: duplicate \"endblock\" for \"{}\"", block_name);
    }

    // Get the positions/indexes of the blocks
    let outer_start: usize = content.find(&capture[0]).unwrap();
    let inner_end: usize = content.find(end_block).unwrap();
    let outer_end: usize = inner_end + end_block.len();
    let inner_start: usize = outer_start + &capture[0].len();

    // Insert the block name and it's compiled HTML into the blocks store
    blocks.insert(block_name.to_string(), Block {
      outer: content[outer_start..outer_end].to_string(),
      inner: marcus::to_string(content[inner_start..inner_end].to_string())
    });
  }

  // Return the given parent template and the blocks
  (RE_EXTENDS.captures(&content).unwrap()[1].to_string(), blocks)
}

/// Get a list of templates containing template data
pub fn list() -> Templates {
  // Get the absolute root of the current project
  let path_root: PathBuf = match project_root() {
    Ok(path) => path,
    Err(error) => panic!("Error: obtaining project root: {:?}", error)
  };

  // Get the absolute dist, public and glob paths
  let path_dist: PathBuf = path_root.join("dist");
  let path_public: PathBuf = path_root.join("public");

  // Get a list of MD templates
  fileio::walk(path_root.join("public/**/*.md").to_str().expect("Error: failed to get glob path"))
    // Convert the file path vector into a vector of templates
    .map(| file_path: String | -> Template {
      // Get the absolute template and output paths
      let path_template: PathBuf = path_root.join(&file_path);
      let path_output: PathBuf = path_dist.join(path_template.strip_prefix(&path_public).expect("Error: failed to strip path prefix"));
      // Read the template's file content and validate the templates
      let (parent, blocks): (String, Blocks) = extract_parent_blocks(&fileio::read_file(&file_path));
      // Return the template struct
      Template { path_template, path_output, parent, blocks }
    })
    // Return the iterator as a vector
    .collect::<Templates>()
}
