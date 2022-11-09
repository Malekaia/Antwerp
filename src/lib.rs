use glob::{glob, GlobError};
use regex::Regex;
use std::{env::current_dir, collections::HashMap, ffi::OsString, fs, io, path::{PathBuf, Ancestors}};

// Type alias for `HashMap<String, Block>`
type Blocks = HashMap<String, String>;

// Used to store and cache regular expressions (prevents multiple definitions of regular expressions)
struct RegExp {
  md_file_extension: Regex,
  extends_statement: Regex,
  block_statement: Regex,
  block_statements_wrapped: Regex
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
    if fs::read_dir(p)?.into_iter().any(| p: Result<fs::DirEntry, io::Error> | {
      p.unwrap().file_name() == OsString::from("Cargo.lock")
    }) {
      return Ok(PathBuf::from(p))
    }
  }
  Err(io::Error::new(io::ErrorKind::NotFound, "could not find Cargo.lock"))
}

// Return an iterator from `glob` (with error handling)
fn walk(path: &str) -> impl Iterator<Item = String> {
  glob(path).expect("GlobError: Failed to read glob pattern").map(| entry: Result<PathBuf, GlobError> |
    entry.expect("GlobError: failed to glob entry").display().to_string()
  )
}

// Extract template data (base template and names/MarkDown templates for blocks)
fn get_base_and_blocks(content: String, re: &RegExp) -> (String, Blocks) {
  // Ensure the template extends a base template
  if !re.extends_statement.is_match(&content) {
    panic!("TemplateError: missing extend.");
  }

  // Store names and MarkDown templates for blocks
  let mut blocks: Blocks = HashMap::new();

  // Iterate all captures for `block` statements
  for capture in re.block_statement.captures_iter(&content) {
    // Get the current block's name
    let block_name: &str = &capture[1];
    // Create a valid `endblock` statement for the current block
    let end_block: &str = &format!("{{% endblock {} %}}", block_name);

    // Ensure all `block` statements have an accompanying `endblock` statement
    if !content.contains(end_block) {
      panic!("TemplateError: missing or mismatched \"endblock\" statement for block (\"{}\")", block_name);
    }
    // Ensure there are no duplicate `block` statements
    else if content.matches(&capture[0]).count() > 1 {
      panic!("TemplateError: duplicate \"block\" for \"{}\"", block_name);
    }
    // Ensure there are no duplicate `endblock` statements
    else if content.matches(end_block).count() > 1 {
      panic!("TemplateError: duplicate \"endblock\" for \"{}\"", block_name);
    }

    // Get the indexes of the (inner) blocks and insert the block's name and content into the `blocks` `HashMap`
    let start: usize = content.find(&capture[0]).unwrap() + &capture[0].len();
    let stop: usize = content.find(end_block).unwrap();
    blocks.insert(block_name.to_string(), content[start..stop].to_string());
  }

  // Return the base template name and the contents of the blocks
  (re.extends_statement.captures(&content).unwrap()[1].to_string(), blocks)
}

/// Compile all `.md` templates in `public/` to `.html` and save them in `dist/`
pub fn build() {
  // Get the root path (absolute) of the project relative to Cargo.lock
  let path_root: PathBuf = match project_root() {
    Ok(path) => path,
    Err(error) => panic!("Error: failed to obtain project root: {:?}", error)
  };

  // Get the `public` and `dist` root paths
  let path_public: PathBuf = path_root.join("public");
  let path_dist: PathBuf = path_root.join("dist");
  // Convert the `public` and `dist` root paths to strings (prevents duplicate conversions in loop)
  let path_public_str: &str = path_public.to_str().unwrap();
  let path_dist_str: &str = path_dist.to_str().unwrap();

  // Cache regular expressions for future referencing
  let re: RegExp = RegExp {
    // Regular expression for `.md` extension replacements
    md_file_extension: Regex::new(r".md$").unwrap(),
    // Regular expression for "extends" statements
    extends_statement: Regex::new(r#"\{% extends "(.*?)" %\}"#).unwrap(),
    // Regular expression for `block` statements
    block_statement: Regex::new(r#"\{% block (.*?) %\}"#).unwrap(),
    // Regular expression for `block` statements
    block_statements_wrapped: Regex::new(r#"\{% block ([^|]+)(\s|\s\|\sraw\s)%\}(.|\n)*?\{% endblock (.*?) %\}"#).unwrap()
  };

  // Iterate the globbed `.md` templates in the `public` directory
  for file_path in walk(&path_root.join("public/**/*.md").to_str().unwrap()) {
    // Get the base template and the blocks from the template
    let template_content: String = fs::read_to_string(&file_path).expect("ReadError: failed to read template file");
    let (mut base, blocks): (String, Blocks) = get_base_and_blocks(template_content, &re);
    base = path_public.join(base).to_str().unwrap().to_string();

    // Get the paths for the: template file, output `.md` file, output file & output directory
    let input: String = path_root.join(&file_path).to_str().unwrap().to_string();
    let output_path: PathBuf = PathBuf::from(input.replace(path_public_str, path_dist_str));
    let output: String = re.md_file_extension.replace(&output_path.to_str().unwrap().to_string(), ".html").to_string();
    let output_dir: String = output_path.parent().unwrap().to_str().unwrap().to_string();

    // Ensure the parent directory for the output `.html` file exists
    fs::create_dir_all(&output_dir).expect("EnsureDirError: failed to create directory");

    // Get the contents of the base template
    let mut html: String = fs::read_to_string(&base).expect("ReadError: failed to read base template file");

    // Iterate the matched blocks in the base template
    for capture in re.block_statements_wrapped.captures_iter(&html.clone()) {
      // Prevent mismatched names in `block` and `endblock` statements
      if &capture[1] != &capture[4] {
        panic!("TemplateError: mismatched name in \"block\" and \"endblock\" statement");
      }
      // Get the block's content as a `String`
      let error_undefined: &str = &format!("TemplateError: block \"{}\" not defined in {:?}", &capture[1], input);
      let output: String = blocks.get(&capture[1]).expect(error_undefined).to_owned();
      // Determine if the MD should be compiled or inserted as raw text
      html = html.replace(&capture[0], &(if capture[2].contains(" | raw") {
        output
      } else {
        marcus::to_string(output)
      }));
    }

    // Write the HTML to the target file
    fs::write(&output, html).expect("WriteError: failed to write to file");
  }
}

// Test build
#[cfg(test)]
mod tests {
  use crate::build;

  #[test]
  fn sample() {
    build();
  }
}
