use glob::glob;
use regex::Regex;
use std::{env::current_dir, ffi::OsString, fs::{DirEntry, read_dir, read_to_string}, io, path::{PathBuf, Ancestors}, collections::HashMap};

const ERROR_READ: &str = "ReadError: Failed to read file";
const ERROR_GLOB: &str = "GlobError: Failed to glob directory";
const ERROR_GLOB_UNWRAP: &str = "GlobError: failed to unwrap PathBuf";
const ERROR_REGEX_UNWRAP: &str = "RegexError: failed to unwrap Regex";
const ERROR_STR_UNWRAP: &str = "RegexError: failed to unwrap &str";
const RE_EXTENDS_STATEMENT: &str = r#"\{%[\s]{0,}extends[\s]{0,}"(.*?)"[\s]{0,}%\}"#;
const RE_PARENT_BLOCK: &str = r#"\{%[\s]{0,}block[\s]{1,}(.*?)[\s]{0,}%\}((.|\n)*?)\{%[\s]{0,}endblock[\s]{1,}(.*?)[\s]{0,}%\}"#;
const RE_MD_EXTENSION: &str = r"\.md$";
const VALID_FILTERS: [&str; 3] = ["text", "html", "trim"];

pub type Filters = Vec<String>;

#[derive(Debug)]
pub struct Block(String, Filters, String);
pub type Blocks = Vec<Block>;

#[derive(Debug)]
pub struct Template {
  pub parent: bool,
  pub extends: String,
  pub output: String,
  pub output_dir: String,
  pub blocks: Blocks
}
pub type Templates = HashMap<String, Template>;

fn project_root() -> io::Result<PathBuf> {
  let path: PathBuf = current_dir()?;
  let mut path_ancestors: Ancestors = path.as_path().ancestors();
  while let Some(p) = path_ancestors.next() {
    if read_dir(p)?.into_iter().any(| p: Result<DirEntry, io::Error> | {
      p.unwrap().file_name() == OsString::from("Cargo.lock")
    }) {
      return Ok(PathBuf::from(p))
    }
  }
  Err(io::Error::new(io::ErrorKind::NotFound, "could not find Cargo.lock"))
}

fn extract_blocks(glob_path: &str) -> Templates {
  // Get the root path (absolute) of the project relative to Cargo.lock
  let path_root: PathBuf = match project_root() {
    Ok(path) => path,
    Err(error) => panic!("Error: failed to obtain project root: {:?}", error)
  };

  // Get the public and dist directories
  let path_dist: PathBuf = path_root.join("dist");
  let path_dist_str: &str = path_dist.to_str().expect(ERROR_STR_UNWRAP);
  let path_public: PathBuf = path_root.join("public");
  let path_public_str: &str = path_public.to_str().expect(ERROR_STR_UNWRAP);

  // Prevent multiple definitions of regular expressions
  let re_extends_statement: Regex = Regex::new(RE_EXTENDS_STATEMENT).expect(ERROR_REGEX_UNWRAP);
  let re_parent_block: Regex = Regex::new(RE_PARENT_BLOCK).expect(ERROR_REGEX_UNWRAP);
  let re_md_extension: Regex = Regex::new(RE_MD_EXTENSION).expect(ERROR_REGEX_UNWRAP);

  // Create a store for the templates
  let mut templates: Templates = HashMap::new();

  // Glob the given path
  for globbed in glob(path_root.join(glob_path).to_str().expect(ERROR_STR_UNWRAP)).expect(ERROR_GLOB) {
    // Get the file_path and file content
    let file_path: String = globbed.expect(ERROR_GLOB_UNWRAP).display().to_string();
    let file_content: String = read_to_string(&file_path).expect(ERROR_READ);

    // Determine if the template extends another
    let mut extends: String = String::new();
    for (i, capture) in re_extends_statement.captures_iter(&file_content).enumerate() {
      if i > 0 {
        panic!("TemplateError: multiple extends statements in \"{}\"", file_path)
      } else if file_path.ends_with(".html") {
        panic!("TemplateError: cannot use a \".html\" file as a child template \"{}\"", file_path);
      }
      extends = path_root.join("public").join(&capture[1].trim()).display().to_string();
    }

    // Get the output path, output directory and parent status
    let parent: bool;
    let output: String;
    let output_dir: String;
    if extends.len() > 0 {
      parent = false;
      output = re_md_extension.replace(&file_path.replace(path_public_str, path_dist_str), ".html").to_string();
      output_dir = PathBuf::from(&output).parent().unwrap().to_str().unwrap().to_string();
    } else {
      parent = true;
      output = String::new();
      output_dir = String::new();
    }

    // Iterate the captured blocks
    let mut blocks: Blocks = vec![];
    for capture in re_parent_block.captures_iter(&file_content) {
      // Get the block name, content and end name
      let (mut name, content, mut end_name): (&str, &str, &str) = (&capture[1].trim(), &capture[2], &capture[4].trim());
      end_name = end_name.trim();
      let mut filters: Filters = vec![];
      // Extract the names and the filters
      for (i, mut item) in name.split("|").enumerate() {
        item = item.trim();
        // The first item is the name of the block
        if i == 0 {
          name = item;
        }
        // All other items are filters
        else {
          // Validate the filters
          if !VALID_FILTERS.contains(&item) {
            panic!("TemplateError: invalid filter (\"{}\") in template \"{}\"", item, file_path);
          }
          filters.push(item.to_string());
        }
      }
      // Ensure the block names match
      if name != end_name {
        panic!("TemplateError: mismatching block names (\"{}\" / \"{}\") in template \"{}\"", name, end_name, file_path);
      }
      // Add the block to the blocks list
      blocks.push(Block(name.to_string(), filters, content.to_string()));
    }

    // Add the template to the templates HashMap
    templates.insert(file_path, Template {
      parent,
      extends,
      output,
      output_dir,
      blocks
    });
  }
  // Return the templates
  templates
}

fn main() {
  let parent_templates: Templates = extract_blocks("public/**/*.html");
  println!("{parent_templates:#?}");
  let child_templates: Templates = extract_blocks("public/**/*.md");
  println!("{child_templates:#?}");
}
