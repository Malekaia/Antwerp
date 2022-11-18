mod filters;
mod parser;
use crate::filters::filter_output;
use crate::parser::parse_templates;
use std::collections::HashMap;
use std::fs::{create_dir_all, write};

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

/// Build parsed templates from `parse::templates()` in `./dist/`
pub fn build() {
  // Get the parent templates
  let parent_templates: Templates = parse_templates("public/**/*.html");

  // Iterate the child templates
  for (file_path, template) in parse_templates("public/**/*.md") {
    // Ensure the parent template exists
    if !parent_templates.contains_key(&template.extends) {
      panic!("TemplateError: unknown template \"{}\"", &template.extends);
    }

    // Get the base template
    let (parent_file_path, parent): (&String, &Template) = parent_templates.get_key_value(&template.extends).unwrap();

    // Create and replace the HTML string
    let mut html: String = parent.content.to_owned();

    // Iterate the blocks
    for (name, block) in &template.blocks {
      // Ensure the block exists in the base template
      if !parent.blocks.contains_key(name) {
        panic!("TemplateError: block \"{}\" in \"{}\" not defined in base template (\"{}\")", name, file_path, parent_file_path);
      }
      // Replace the block content with the given template
      let parent_block: &Block = parent.blocks.get(name).unwrap();
      html = html.replace(&parent_block.content_outer, &filter_output(&block.filters, &block.content));
    }

    // Iterate the parent blocks
    for (_, block) in &parent.blocks {
      // Replace unspecified blocks with their default value
      if html.contains(&block.content_outer) {
        html = html.replace(&block.content_outer, &filter_output(&block.filters, &block.content));
      }
    }

    // Create the output file's directory
    match create_dir_all(&template.output_dir) {
      Ok(_) => {},
      Err(_) => panic!("CreateDirAllError: failed to create directory \"{}\"", &template.output_dir)
    };

    // Write the HTML to the output file
    match write(&template.output, &html) {
      Ok(_) => {},
      Err(_) => panic!("WriteError: write \"html\" to \"{}\"", &template.output)
    };
  }
}

#[cfg(test)]
mod tests {
  use crate as Antwerp;
  #[test]
  fn antwerp() {
    Antwerp::build();
  }
}
