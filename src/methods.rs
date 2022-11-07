use crate::fileio;
use marcus;
use regex::Regex;
use std::collections::HashMap;

type HashMapBlocks = HashMap<String, (String, String)>;

#[derive(Debug)]
pub struct Template {
  pub path: String,
  pub parent: String,
  pub blocks: HashMapBlocks
}

pub fn get_templates() -> Vec<Template> {
  // Create a store for the templates
  let mut templates: Vec<Template> = vec![];

  // Regular expression for "extends" declarations
  let re_extends: Regex = Regex::new(r#"\{% extends "(.*?)" %\}"#).unwrap();

  // Regular expression for "block" declarations
  let re_block: Regex = Regex::new(r#"\{% block (.*?) %\}"#).unwrap();

  // Glob the MD template files
  for file_path in fileio::walk("./public/**/*.md") {
    // Read the template's file content
    let content: String = fileio::read_file(&file_path);

    // Ensure the template extends a parent template
    if !re_extends.is_match(&content) {
      panic!("TemplateError: missing extend.");
    }

    // Get the parent template name
    let parent_template: &str = &re_extends.captures(&content).unwrap()[1];

    // Create a store for the block names and their corresponding converted HTML
    let mut blocks: HashMapBlocks = HashMap::new();

    // Capture and extract blocks
    for capture in re_block.captures_iter(&content) {
      // Get the block name
      let block_name: &str = &capture[1];

      // Ensure all "block" declarations have an accompanying "endblock" declaration
      let end_block: &str = &format!("{{% endblock {} %}}", block_name);
      if !content.contains(end_block) {
        panic!("TemplateError: missing \"endblock\" for {}", block_name);
      }

      // Get the wrapped block (surrounded by "block" and "endblock" declarations)
      let block_wrapped: &str = &content[content.find(&capture[0]).unwrap()..(content.find(end_block).unwrap() + end_block.len())];

      // Get the inner block (without the "block" and "endblock")
      let block_inner: &str = &content[(content.find(&capture[0]).unwrap() + &capture[0].len())..content.find(end_block).unwrap()];

      // Insert the block name and it's compiled HTML into the blocks store
      blocks.insert(block_name.to_string(), (block_wrapped.to_string(), marcus::to_string(block_inner.to_string())));
    }

    // Add the template to the templates list
    templates.push(Template {
      path: file_path,
      parent: parent_template.to_string(),
      blocks,
    });
  }

  // Return the templates vector
  templates
}
