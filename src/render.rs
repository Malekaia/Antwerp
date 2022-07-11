use crate::lib::{file_system, walk};
use grass;
use regex::Regex;
use tera::{Context, Tera};

pub struct CopyDetails<'a> (pub &'a str, pub &'a str, pub bool);

pub struct Render<'a> {
  pub verbose: bool,
  pub empty_dist: bool,
  pub tera: Tera,
  pub empty_context: Context,
  pub copy_dirs: Vec<CopyDetails<'a>>,
  pub copy_files: Vec<CopyDetails<'a>>,
  pub assets_301: Vec<[&'a str; 2]>,
  pub assets_410: Vec<&'a str>,
  pub scss_assets: Vec<[&'a str; 2]>
}

impl Render<'_> {
  fn copy_dirs(&self) {
    for copy_config in &self.copy_dirs {
      let source: &str = copy_config.0;
      let check: &str = copy_config.1;
      let overwrite: bool = copy_config.2;
      let re_check: Regex = Regex::new(&check).expect(&format!("Error: failed to create regex: {}", &check));
      for path in walk::dir(source) {
        if !re_check.is_match(&path) {
          continue;
        }
        if self.verbose == true {
          let overwrite_status: &str = if overwrite == true { "overwrite" } else { "no overwrite" };
          println!("Copy ({}): {}", overwrite_status, &path);
        }
        let destination: String = path.clone().replace("public/", "dist/");
        file_system::copy_file(&path, &destination, overwrite);
      }
    }
  }

  fn copy_files(&self) {
    for copy_config in &self.copy_files {
      let source: &str = copy_config.0;
      let destination: &str = copy_config.1;
      let overwrite: bool = copy_config.2;
      if self.verbose == true {
        let overwrite_status: &str = if overwrite == true { "overwrite" } else { "no overwrite" };
        println!("Copy ({}): {}", overwrite_status, &source);
      }
      file_system::copy_file(&source, &destination, overwrite);
    }
  }

  fn template_to_file(&self, name: &str, destination: &str, context: &Context) {
    let result: String = self.template_to_string(&name, &context);
    file_system::write_file(&destination, &result);
  }

  fn template_to_string(&self, name: &str, context: &Context) -> String {
    match self.tera.render(&name, &context) {
      Ok(result) => result,
      Err(error) => panic!("Error: failed to render tera template:\n\n{}", error)
    }
  }

  pub fn empty_dist(&self) {
    if self.empty_dist == true {
      if self.verbose == true {
        println!("Delete: ./dist");
      }
      file_system::empty_dir("./dist/");
    }
  }

  pub fn copy_static(&self) {
    self.copy_dirs();
    self.copy_files();
  }

  pub fn compile_scss(&self) {
    for [source, destination] in &self.scss_assets {
      let result: String = match grass::from_path(source, &grass::Options::default()) {
        Ok(result) => result,
        Err(error) => panic!("Error: failed to compile SASS stylesheets\n\n{:?}", error)
      };
      if self.verbose == true {
        println!("Compile (SCSS): {}", &destination);
      }
      file_system::write_file(&destination, &result);
    }
  }

  pub fn tera_instance(dir_templates: &str) -> Tera {
    let mut tera: Tera = match Tera::parse(dir_templates) {
      Ok(tera_object) => tera_object,
      Err(error) => panic!("Error: failed to render \"tera\" template:\n\n{}", error)
    };
    tera.build_inheritance_chains().expect("Error: failed to build tera inheritance chain");
    tera
  }

  pub fn template(&self, name: &str, destination: &str, context: &Context) {
    if self.verbose == true {
      println!("Render (route): {}", &destination);
    }
    self.template_to_file(name, destination, context);
  }

  pub fn template_string(&mut self, input: &String, context: &Context) -> String {
    match self.tera.render_str(&input, &context) {
      Ok(result) => result,
      Err(error) => panic!("Error: failed to render tera string:\n\n{}", error)
    }
  }

  pub fn assets_301(&self, template: &str) {
    let mut context_301: Context = Context::new();
    for [url, redirect] in &self.assets_301 {
      if self.verbose == true {
        println!("Render (301): {}", &template);
      }
      context_301.insert("redirect", redirect);
      self.template_to_file(&template, url, &context_301);
    }
  }

  pub fn assets_410(&self, template: &str) {
    for url in &self.assets_410 {
      if self.verbose == true {
        println!("Render (410): {}", url);
      }
      self.template_to_file(template, url, &self.empty_context);
    }
  }
}