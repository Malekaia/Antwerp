use crate::antwerp::{Config, Lib};
use tera::{Context, Tera};

/// **Description**:
///
/// Used to define template information in a template group, defines the output file and the context
pub struct Template<'a> {
  pub output: &'a str,
  pub context: Context
}


/// **Description**:
///
/// Returns a new instance of Tera and builds it's inheritance chains
pub fn tera(template_directory: &str) -> Tera {
  // Try to create the tera instance
  let mut tera: Tera = match Tera::parse(template_directory) {
    Ok(tera_object) => tera_object,
    Err(error) => panic!("Error: failed to render \"tera\" template:\n\n{}", error)
  };
  // Try to build the inheritance chains
  tera.build_inheritance_chains().expect("Error: failed to build tera inheritance chain");
  tera
}


/// **Description**:
///
/// Renders a Tera template using a given context and writes it an output file
pub fn render(tera: &Tera, template_path: &str, output: &str, context: &Context) {
  // Try to render the template
  match tera.render(template_path, context) {
    // Write the rendered template to the output file
    Ok(result) => {
      Lib::write_file(output, result);
    },
    // Panic and exit
    Err(error) => panic!("Error: failed to render tera template:\n\n{}", error)
  };
}


/// **Description**:
///
/// Takes a string containing a Tera template string and returns a string containing the rendered template
pub fn render_string(tera: &mut Tera, template_string: &String, context: Context) -> String {
  // Try to render the string and return it (panic on fail)
  match tera.render_str(template_string, &context) {
    Ok(result) => result,
    Err(error) => panic!("Error: failed to render tera string:\n\n{}", error)
  }
}


/// **Description**:
///
/// Takes template information (template name, output and context) and renders it to a file
pub fn route(config: &Config, template_name: &str, output: &str, context: &Context) {
  // Log the update
  Lib::log(config.verbose, "green", "Render", "static", output);
  // Render the given template
  render(&config.tera, template_name, output, context);
}


/// **Description**:
///
/// Takes a Vector containing Template(s) (`Vec<Template>`) and passes each template to `lib::route`
pub fn route_group(config: &Config, template_name: &str, templates: &Vec<Template>) {
  for template in templates {
    // Log the update
    Lib::log(config.verbose, "green", "Render", "static", template.output);
    // Render the current template
    render(&config.tera, template_name, template.output, &template.context);
  }
}