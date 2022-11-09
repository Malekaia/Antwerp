use crate::fileio;
use marcus;
use regex::Regex;
use std::path::Path;
use crate::template::Templates;

// Define regular expressions once
lazy_static! {
  /// Regular expression for "block" statements
  static ref RE_BLOCKS: Regex = Regex::new(r#"\{% block ([^|]+)(\s\|\sraw\s|\s)%\}(.|\n)*?\{% endblock (.*?) %\}"#).unwrap();
}

// Build the output templates
pub fn templates(templates: Templates) {
  // Iterate the templates
  for template in templates {
    // Create the directory for the output `.html` file
    fileio::ensure_dir(Path::new(&template.path_output).parent().unwrap().to_str().unwrap());
    // Get the contents of the parent template
    let template_content: String = fileio::read_file(&template.path_parent.to_str().unwrap());
    let mut html: String = template_content.to_owned();
    // Iterate the given blocks
    for capture in RE_BLOCKS.captures_iter(&template_content) {
      // Prevent mismatched "block" and "endblock" names
      if &capture[1] != &capture[4] {
        panic!("Error: mismatched name in \"block\" and \"endblock\" statement");
      }
      // Insert the inner block into the HTML
      let error_undefined: &str = &format!("Error: block \"{}\" not defined in {:?}", &capture[1], &template.path_template);
      // Compile the output or insert it as raw text
      let output: &str = &template.blocks.get(&capture[1]).expect(error_undefined).inner;
      html = html.replace(&capture[0], &(if capture[2].contains(" | raw") {
        output.to_string()
      } else {
        marcus::to_string(output.to_string())
      }));
    }
    // Write the HTML to the target file
    fileio::write_file(&template.path_output.to_str().unwrap(), html);
  }
}
