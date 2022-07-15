#[allow(non_snake_case)]
pub mod Antwerp {
  use crate::lib::{file_system, walk};
  use colored::Colorize;
  use regex::Regex;
  use tera::{Context, Tera};

  fn print(color: &str, action: &str, category: &str, target: &str) {
    println!("{} ({}): {}", format!("{action}").color(color), format!("{category}").bold(), target);
  }

  pub fn empty_root(root: &str) {
    {
      print("red", "Empty", "folder", root);
    }
    file_system::empty_dir(root);
  }

  pub struct Template<'a> {
    pub output: &'a str,
    pub context: Context
  }

  pub fn tera(template_directory: &str) -> Tera {
    let mut tera: Tera = match Tera::parse(template_directory) {
      Ok(tera_object) => tera_object,
      Err(error) => panic!("Error: failed to render \"tera\" template:\n\n{}", error)
    };
    tera.build_inheritance_chains().expect("Error: failed to build tera inheritance chain");
    tera
  }

  pub fn render(tera: &Tera, template_path: &str, output: &str, context: Context) {
    let result: String = match tera.render(template_path, &context) {
      Ok(result) => result,
      Err(error) => panic!("Error: failed to render tera template:\n\n{}", error)
    };
    file_system::write_file(output, &result);
  }

  pub fn render_string(tera: &mut Tera, template: &String, context: Context) -> String {
    match tera.render_str(&template, &context) {
      Ok(result) => result,
      Err(error) => panic!("Error: failed to render tera string:\n\n{}", error)
    }
  }

  pub fn route(tera: &Tera, template: &str, output: &str, context: Context) {
    {
      print("green", "Render", "static", &output);
    }
    render(tera, template, output, context);
  }

  pub fn route_group(tera: &Tera, template_path: &str, templates: Vec<Template>) {
    for template in templates {
      {
        print("green", "Render", "static", &template.output);
      }
      render(tera, template_path, template.output, template.context);
    }
  }

  pub enum Asset<'a> {
    File(&'a str, &'a str, bool),
    Folder(&'a str, &'a str, &'a str, bool),
    Scss(&'a str, &'a str),
  }

  pub fn assets(assets: Vec<Asset>) {
    for asset in assets {
      match asset {
        // copy files
        Asset::File(source, destination, overwrite) => {
          {
            let overwrite_status: &str = if overwrite == true { "overwrite: false" } else { "overwrite: true" };
            print("blue", "Copy", overwrite_status, &destination);
          }
          file_system::copy_file(&source, &destination, overwrite);
        },

        // copy (recursive) directories
        Asset::Folder(source, mut destination, check, overwrite) => {
          let re_root: Regex = Regex::new(r"^(.*?)/(.*?)$").unwrap();
          let re_check: Regex = Regex::new(&check).expect(&format!("Error: failed to create regex: {}", &check));
          for path in walk::dir(source) {
            if !re_check.is_match(&path) {
              continue;
            }
            destination = if destination.ends_with("/") { &destination[0..(destination.len() - 1)] } else { destination };
            let dest: String = re_root.replace(&*path, format!("{}/$2", destination)).to_string();
            {
              let overwrite_status: &str = if overwrite == true { "overwrite: false" } else { "overwrite: true" };
              print("blue", "Copy", overwrite_status, &dest);
            }
            file_system::copy_file(&path, &dest, overwrite);
          }
        },

        // compile SCSS assets
        Asset::Scss(source, destination) => {
          let result: String = match grass::from_path(source, &grass::Options::default()) {
            Ok(result) => result,
            Err(error) => panic!("Error: failed to compile SASS stylesheets\n\n{:?}", error)
          };
          {
            print("magenta", "Compile", "SCSS", &destination);
          }
          file_system::write_file(&destination, &result);
        }
      }
    }
  }
}