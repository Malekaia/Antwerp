use crate::{Lib, Post};
use regex::Regex;
use tera::{Context, Tera};
use chrono::{Timelike, DateTime, Datelike};
use chrono::prelude::Local;
use fs_extra::dir::{move_dir, CopyOptions};

pub struct Antwerp {
  pub tera: Option<Tera>,
  pub empty_context: Context,
  pub clean: bool,
  pub safe_clean: bool,
  pub verbose: bool,
  pub url_root: String,
  pub url_post: String,
  pub dir_resources: String,
  pub dir_output: String,
  pub dir_posts: String,
  pub path_render: String,
  pub post_list: Vec<Post>
}

impl Antwerp {
  pub fn default() -> Antwerp {
    Antwerp {
      tera: None,
      empty_context: Context::new(),
      clean: false,
      safe_clean: true,
      verbose: true,
      url_root: String::new(),
      url_post: String::new(),
      dir_resources: String::new(),
      dir_output: String::new(),
      dir_posts: String::new(),
      path_render: String::new(),
      post_list: vec![]
    }
  }

  pub fn clean(&mut self, clean: bool, safe_clean: bool) {
    self.clean = clean;
    self.safe_clean = safe_clean;
    // Ensure `dir_output` has been defined
    if self.dir_output.trim().len() < 1 {
      panic!("Error: Cannot clean build, `dir_output` has not been defined for this build instance.");
    }
    // Only clean if requested
    if clean == true {
      // Safe clean by default
      if self.safe_clean == true {
        // Log if verbose is enabled
        Lib::log(self.verbose, "yellow", "Clean", "folder", &self.dir_output);
        // Create the folder name
        let date: DateTime<Local> = Local::now();
        let folder_name: String = format!(
          "{}-{:0>2}-{:0>2}-at-{:0>2}-{:0>2}-{:0>2}",
          date.year(), date.month(), date.day(), date.hour(), date.minute(), date.second()
        );
        let path_move: &str = &format!("./.antwerp/Trash/{}/", folder_name);
        // Move the directory to .antwerp/Trash
        Lib::ensure_dir("./.antwerp/Trash");
        Lib::ensure_dir(path_move);
        // Move the directory into it's storage container
        let options: CopyOptions = CopyOptions::new();
        move_dir(&self.dir_output, path_move, &options).expect("Error: failed to move folder during clean");
        // Ensure the directory exists after move
        Lib::ensure_dir(&self.dir_output);
      }
      // Warning! This empties a directory by permanently deleting its contents!
      else {
        // Log if verbose is enabled
        Lib::log(self.verbose, "red", "Delete", "folder", &self.dir_output);
        // empty the directory
        Lib::empty_dir(&self.dir_output);
      }
    }
  }

  pub fn tera(&mut self, template_directory: &str) {
    let t_d: String = Lib::path_from_cwd(template_directory);
    // Try to create the tera instance
    let mut tera: Tera = match Tera::parse(&t_d) {
      Ok(tera_object) => tera_object,
      Err(error) => panic!("Error: failed to render \"tera\" template:\n\n{}", error)
    };

    // Try to build the inheritance chains
    tera.build_inheritance_chains().expect("Error: failed to build tera inheritance chain");
    self.tera = Some(tera);
  }

  pub fn verbose(&mut self, verbose: bool) {
    self.verbose = verbose;
  }

  pub fn url_root(&mut self, url_root: &str) {
    self.url_root = url_root.to_string();
  }

  pub fn url_post(&mut self, url_post: &str) {
    self.url_post = url_post.to_string();
  }

  pub fn dir_resources(&mut self, dir_resources: &str) {
    let d_r: String = Lib::path_absolute(dir_resources);
    Lib::ensure_dir(&d_r);
    self.dir_resources = d_r;
  }

  pub fn dir_output(&mut self, dir_output: &str) {
    let d_o: String = Lib::path_absolute(dir_output);
    Lib::ensure_dir(&d_o);
    self.dir_output = d_o;
  }

  pub fn dir_posts(&mut self, dir_posts: &str) {
    self.dir_posts = dir_posts.to_string();
  }

  pub fn path_render(&mut self, path_render: &str) {
    self.path_render = path_render.to_string();
  }

  pub fn post_list(&mut self, sorter: fn(Vec<Post>) -> Vec<Post>) {
    self.post_list = Post::list(&self, sorter)
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
      let from: &str = &Lib::path_from_cwd(&path_string);
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
}
