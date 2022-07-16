#[allow(non_snake_case)]
pub mod Antwerp {
  use crate::lib;
  use colored::Colorize;
  use regex::Regex;
  use tera::{Context, Tera};


  /// **Description**:
  ///
  /// Log a colourised string about a certain action
  fn print(color: &str, action: &str, category: &str, target: &str) {
    println!("{} ({}): {}", format!("{action}").color(color), format!("{category}").bold(), target);
  }


  /// **Description**:
  ///
  /// Empty the root folder (see `lib::empty_dir`)
  pub fn empty_root(root: &str) {
    // Log the update
    print("red", "Empty", "folder", root);
    // Empty the directory
    lib::empty_dir(root);
  }


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
  pub fn render(tera: &Tera, template_path: &str, output: &str, context: Context) {
    // Try to render the template
    let result: String = match tera.render(template_path, &context) {
      Ok(result) => result,
      Err(error) => panic!("Error: failed to render tera template:\n\n{}", error)
    };
    // Write the rendered template to the output file
    lib::write_file(output, &result);
  }


  /// **Description**:
  ///
  /// Takes a string containing a Tera template string and returns a string containing the rendered template
  pub fn render_string(tera: &mut Tera, template_string: &String, context: Context) -> String {
    // Try to render the string and return it (panic on fail)
    match tera.render_str(&template_string, &context) {
      Ok(result) => result,
      Err(error) => panic!("Error: failed to render tera string:\n\n{}", error)
    }
  }


  /// **Description**:
  ///
  /// Takes template information (template name, output and context) and renders it to a file
  pub fn route(tera: &Tera, template_name: &str, output: &str, context: Context) {
    // Log the update
    print("green", "Render", "static", &output);
    // Render the given template
    self::render(tera, template_name, output, context);
  }


  /// **Description**:
  ///
  /// Takes a Vector containing Template(s) (`Vec<Template>`) and passes each template to `lib::route`
  pub fn route_group(tera: &Tera, template_name: &str, templates: Vec<Template>) {
    for template in templates {
      // Log the update
      print("green", "Render", "static", &template.output);
      // Render the current template
      self::render(tera, template_name, template.output, template.context);
    }
  }


  /// **Description**:
  ///
  /// An enum used to define data for `File`, `Folder`, and `Scss` assets for `Antwerp::assets([...])`.
  ///
  /// **Usage**:
  ///
  /// `Asset::File(source: &str, destination: &str, overwrite: bool)`
  ///
  /// `Asset::Folder(source: &str, destination: &str, check: &str, overwrite: bool)`
  ///
  /// `Asset::Scss(source: &str, destination &str)`
  pub enum Asset<'a> {
    // source, destination, overwrite
    File(&'a str, &'a str, bool),
    // source, destination, check, overwrite
    Folder(&'a str, &'a str, &'a str, bool),
    // source, destination
    Scss(&'a str, &'a str),
  }


  /// **Description**:
  ///
  /// Takes asset definitions using a `Vec<Asset>` and uses match to determine how to copy or compile those assets to their target destination
  ///
  /// **Behaviour**:
  ///
  /// * Copies individual files to a specified file path
  /// * Copies files in specified folders that pass a check
  /// * Compiles SCSS stylesheets to a specified file path
  ///
  /// **References**:
  ///
  /// * [Match (Flow Control)](https://doc.rust-lang.org/rust-by-example/flow_control/match.html)
  pub fn assets(assets: Vec<Asset>) {
    for asset in assets {
      // Handle different asset types
      match asset {
        // Copy static assets
        Asset::File(source, destination, overwrite) => {
          // Log the update
          let overwrite_status: &str = if overwrite == true { "overwrite: false" } else { "overwrite: true" };
          print("blue", "Copy", overwrite_status, &destination);

          // Copy the file to the intended destination
          lib::copy_file(&source, &destination, overwrite);
        },

        // Copy (recursive) directories
        Asset::Folder(source, mut destination, check, overwrite) => {
          // Regular expression used to remove the root of the source path
          let re_root: Regex = Regex::new(r"^(.*?)/(.*?)$").unwrap();
          // Regular expression used to check the matches
          let re_check: Regex = Regex::new(check).expect(&format!("Error: failed to create regex: {}", check));

          for path in lib::walk_dir(source) {
            // Ignore the match if it doesn't pass the check
            if !re_check.is_match(&path) {
              continue;
            }
            // Remove trailing slashes from the destination path
            if destination.ends_with("/") {
              destination = &destination[0..(destination.len() - 1)]
            };
            // Create the destination path (without the source root)
            let new_destination: &str = &*re_root.replace(&path, format!("{destination}/$2")).to_string();
            // Log the update
            let overwrite_status: &str = if overwrite == true { "overwrite: false" } else { "overwrite: true" };
            print("blue", "Copy", overwrite_status, new_destination);
            // Copy the file
            lib::copy_file(&path, new_destination, overwrite);
          }

        },

        // Compile SCSS assets
        Asset::Scss(source, destination) => {
          // Try to compile the SCSS stylesheet (return if possible, panic if not)
          let result: String = match grass::from_path(source, &grass::Options::default()) {
            Ok(result) => result,
            Err(error) => panic!("Error: failed to compile SASS stylesheets\n\n{:?}", error)
          };
          // Log the update
          print("magenta", "Compile", "SCSS", &destination);
          // Write the compiled SCSS to the destination file
          lib::write_file(&destination, &result);
        }
      }
    }
  }
}