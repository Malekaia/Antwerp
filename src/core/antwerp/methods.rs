use crate::{Antwerp, Lib, Post};
use regex::Regex;
use tera::{Context, Tera};

impl Antwerp {
  pub fn post_list(&self, sorter: fn(Vec<Post>) -> Vec<Post>) -> Vec<Post> {
    Post::list(&self, sorter)
  }

  pub fn folder(&self, source: &str, check: &str, overwrite: bool) {
    // Regular expression used to check the matches
    let re_check: Regex = Regex::new(check).expect(&format!("Error: failed to create regex: {}", check));
    // Create required paths
    let search_path: &str = &Lib::path_join(&self.dir_resources, source);
    // Iterate the file paths
    for path_string in Lib::walk_dir(search_path) {
      // Ignore the match if it doesn't pass the check
      if !re_check.is_match(&path_string) {
        continue;
      }
      // Create the copy to and from paths
      let from: &str = &Lib::path_absolute(&path_string);
      let to: &str = &from.replace(&self.dir_resources, &self.dir_output);
      // Log the update
      let overwrite_status: &str = if overwrite == true { "overwrite: false" } else { "overwrite: true" };
      let mut to_display: &str = &to.replace(&self.dir_output, "");
      to_display = if to_display.starts_with("/") { &to_display[1..to_display.len()] } else { to_display };
      Lib::log(self.verbose, "blue", "Copy", overwrite_status, to_display);
      // Copy the file
      Lib::copy_file(from, to, overwrite);
    }
  }

  pub fn file(&self, source: &str, destination: &str, overwrite: bool) {
    // Create the from and to paths
    let from: &str = &Lib::path_join(&self.dir_resources, source);
    let to: &str = &Lib::path_join(&self.dir_output, destination);
    // Log the update
    let overwrite_status: &str = if overwrite == true { "overwrite: false" } else { "overwrite: true" };
    Lib::log(self.verbose, "blue", "Copy", overwrite_status, destination);
    // Copy the file to the intended destination
    Lib::copy_file(from, to, overwrite);
  }

  pub fn scss(&self, source: &str, destination: &str) {
    // Create the from and to paths
    let from: &str = &Lib::path_join(&self.dir_resources, source);
    let to: &str = &Lib::path_join(&self.dir_output, destination);
    // Try to compile the SCSS stylesheet (return if possible, panic if not)
    match grass::from_path(from, &grass::Options::default()) {
      // Write the file if successful
      Ok(result) => {
        // Log the update
        Lib::log(self.verbose, "magenta", "Compile", "SCSS", destination);
        // Write the compiled SCSS to the destination file
        Lib::write_file(to, result);
      },
      // Panic and exit
      Err(error) => panic!("Error: failed to compile SASS stylesheets\n\n{:?}", error)
    };
  }

  pub fn route(&self, template_name: &str, output: &str, context: &Context) {
    // Create required paths
    let to: &str = &Lib::path_join(&self.dir_output, output);
    let to_printed: String = to.replace(&self.dir_output, "");
    // Log the update
    Lib::log(self.verbose, "green", "Render", "static", &to_printed);
    // Try to render the given template
    match self.tera.as_ref().unwrap().render(template_name, &context) {
      // Write the rendered template to the output file
      Ok(result) => Lib::write_file(to, result),
      // Panic and exit
      Err(error) => panic!("Error: failed to render tera template:\n\n{}", error)
    };
  }

  pub fn render(&self, template_name: &String, context: &Context) -> String {
    // Try to render the string and return it (panic on fail)
    match self.tera.as_ref().unwrap().render(template_name, &context) {
      // Write the rendered template to the output file
      Ok(result) => result,
      // Panic and exit
      Err(error) => panic!("Error: failed to render tera template:\n\n{}", error)
    }
  }

  // Note: this method can be inefficient in a loop due to use of "clone"
  pub fn render_string(&self, template_name: &String, context: &Context) -> String {
    let tera: Option<Tera> = self.tera.clone();
    tera.unwrap().render_str(&template_name, &context)
      .expect(&format!("Error: failed to render string for \"{template_name}\""))
  }
}