use crate::filter;
use crate::parse::templates;
use crate::types::{Block, Template, Templates};
use std::fs::{create_dir_all, write};

/// Build parsed templates from `parse::templates()` in `./dist/`
pub fn dist() {
  // Get the parent templates
  let parent_templates: Templates = templates("public/**/*.html");

  // Iterate the child templates
  for (file_path, template) in templates("public/**/*.md") {
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
      html = html.replace(&parent_block.content_outer, &filter::output(&block.filters, &block.content));
    }

    // Iterate the parent blocks
    for (_, block) in &parent.blocks {
      // Replace unspecified blocks with their default value
      if html.contains(&block.content_outer) {
        html = html.replace(&block.content_outer, &filter::output(&block.filters, &block.content));
      }
    }


    match create_dir_all(&template.output_dir) {
      Ok(_) => {},
      Err(_) => panic!("CreateDirAllError: failed to create directory \"{}\"", &template.output_dir)
    };
    match write(&template.output, &html) {
      Ok(_) => {},
      Err(_) => panic!("WriteError: write \"html\" to \"{}\"", &template.output)
    };
  }
}
