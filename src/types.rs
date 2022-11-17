use std::collections::HashMap;

/// Error for `fs` file read operations
pub const ERROR_READ: &str = "ReadError: Failed to read file";
/// Error for directory `glob` errors
pub const ERROR_GLOB: &str = "GlobError: Failed to glob directory";
/// Error for `glob` unwrap errors
pub const ERROR_GLOB_UNWRAP: &str = "GlobError: failed to unwrap PathBuf";
/// Error for `Regex` unwrap error
pub const ERROR_REGEX_UNWRAP: &str = "RegexError: failed to unwrap Regex";
/// Error for `&str` unwrap error
pub const ERROR_STR_UNWRAP: &str = "RegexError: failed to unwrap &str";
/// Regular expression for extends (`{% extends "..." %}`) statements
pub const RE_EXTENDS_STATEMENT: &str = r#"\{%[\s]{0,}extends[\s]{0,}"(.*?)"[\s]{0,}%\}"#;
/// Regular expression for block (`{% block ... | ... %}...{% endblock ... %}`) statements
pub const RE_PARENT_BLOCK: &str = r#"\{%[\s]{0,}block[\s]{1,}(.*?)[\s]{0,}%\}((.|\n)*?)\{%[\s]{0,}endblock[\s]{1,}(.*?)[\s]{0,}%\}"#;
/// Regular expression for `.md` file types
pub const RE_MD_EXTENSION: &str = r"\.md$";
/// List of valid filters for blocks
pub const VALID_FILTERS: [&str; 3] = ["text", "html", "trim"];

/// Type alias for filters in block statements (`Vector` of `String`)
pub type Filters = Vec<String>;

/// Type for blocks in templates
#[derive(Debug)]
pub struct Block {
  pub filters: Filters,
  pub content_outer: String,
  pub content: String
}

/// Type alias for a `Vector` of `Block`
pub type Blocks = HashMap<String, Block>;

/// Type for templates
#[derive(Debug)]
pub struct Template {
  pub extends: String,
  pub output: String,
  pub output_dir: String,
  pub content: String,
  pub blocks: Blocks
}

/// Type alias for `HashMap` with `String` for keys and `Template` for values
pub type Templates = HashMap<String, Template>;
