use grass;
use crate::lib::{FileSystem, Walk};
use regex::Regex;
use tera::{Context, Tera};

pub struct CopyDetails<'a> (pub &'a str, pub &'a str, pub bool);

pub struct Render<'a> {
  pub verbose: bool,
  pub clear_on_start: bool,
  pub tera: Tera,
  pub empty_context: Context,
  pub copy_dirs: Vec<CopyDetails<'a>>,
  pub copy_files: Vec<CopyDetails<'a>>,
  pub http_301_assets: Vec<[&'a str; 2]>,
  pub http_410_assets: Vec<&'a str>,
  pub scss_assets: Vec<[&'a str; 2]>
}

impl Render<'_> {
  pub fn new(dir_templates: &str) -> Render {
    Render {
      verbose: true,
      clear_on_start: true,
      tera: {
        let mut tera: Tera = match Tera::parse(&dir_templates) {
          Ok(tera_object) => tera_object,
          Err(error) => panic!("Error: failed to render \"tera\" template:\n\n{}", error)
        };
        tera.build_inheritance_chains().expect("Error: failed to build tera inheritance chain");
        tera
      },
      empty_context: Context::new(),
      copy_dirs: vec![],
      copy_files: vec![],
      http_301_assets: vec![],
      http_410_assets: vec![],
      scss_assets: vec![]
    }
  }

  pub fn clear_on_start(&self) {
    if self.clear_on_start == true {
      if self.verbose == true {
        println!("Delete: ./dist");
      }
      FileSystem::empty_dir("./dist/");
    }
  }

  pub fn copy_dirs(&self) {
    for copy_config in &self.copy_dirs {
      let source: &str = copy_config.0;
      let check: &str = copy_config.1;
      let overwrite: bool = copy_config.2;
      let re_check: Regex = Regex::new(&check).expect(&format!("Error: failed to create regex: {}", &check));
      for path in Walk::dir(source) {
        if !re_check.is_match(&path) {
          continue;
        }
        if self.verbose == true {
          let overwrite_status: &str = if overwrite == true { "overwrite" } else { "no overwrite" };
          println!("Copy ({}): {}", overwrite_status, &path);
        }
        let destination: String = path.clone().replace("public/", "dist/");
        FileSystem::copy_file(&path, &destination, overwrite);
      }
    }
  }

  pub fn copy_files(&self) {
    for copy_config in &self.copy_files {
      let source: &str = copy_config.0;
      let destination: &str = copy_config.1;
      let overwrite: bool = copy_config.2;
      if self.verbose == true {
        let overwrite_status: &str = if overwrite == true { "overwrite" } else { "no overwrite" };
        println!("Copy ({}): {}", overwrite_status, &source);
      }
      FileSystem::copy_file(&source, &destination, overwrite);
    }
  }

  fn template_to_string(&self, name: &str, context: &Context) -> String {
    match self.tera.render(&name, &context) {
      Ok(result) => result,
      Err(error) => panic!("Error: failed to render tera template:\n\n{}", error)
    }
  }

  fn template_to_file(&self, name: &str, destination: &str, context: &Context) {
    let result: String = self.template_to_string(&name, &context);
    FileSystem::write_file(&destination, &result);
  }

  fn template_from_string(&mut self, input: &String, context: &Context) -> String {
    match self.tera.render_str(&input, &context) {
      Ok(result) => result,
      Err(error) => panic!("Error: failed to render tera string:\n\n{}", error)
    }
  }

  pub fn template(&self, name: &str, destination: &str, context: &Context) {
    if self.verbose == true {
      println!("Render (route): {}", &destination);
    }
    self.template_to_file(name, destination, context);
  }

  pub fn template_string(&mut self, name: &str, input: &String, context: &Context) -> String {
    if self.verbose == true {
      println!("Render (tera): {}", name);
    }
    self.template_from_string(input, context)
  }

  pub fn render_301(&self, template: &str) {
    let mut context_301: Context = Context::new();
    for [url, redirect] in &self.http_301_assets {
      if self.verbose == true {
        println!("Render (301): {}", &template);
      }
      context_301.insert("redirect", redirect);
      self.template_to_file(&template, url, &context_301);
    }
  }

  pub fn render_410(&self, template: &str) {
    for url in &self.http_410_assets {
      if self.verbose == true {
        println!("Render (410): {}", url);
      }
      self.template_to_file(template, url, &self.empty_context);
    }
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
      FileSystem::write_file(&destination, &result);
    }
  }
}