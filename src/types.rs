use std::collections::HashMap;


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
