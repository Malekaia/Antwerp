use crate::{Antwerp, Config, Lib};
use chrono::{{Timelike, DateTime, Datelike}, prelude::Local};
use fs_extra::dir::{move_dir, CopyOptions};
use tera::{Context, Tera};
use toml as TOML;

impl Antwerp {
  pub fn new() -> Antwerp {
    if !Lib::exists("./antwerp.toml") {
      panic!("Error: cannot find antwerp.toml!");
    }

    let config: Config = TOML::from_str(&Lib::read_file("./antwerp.toml")).unwrap();

    // FIXME: remove need for `to_owned`
    let mut build: Antwerp = Antwerp {
      config: config.to_owned(),
      tera: None,
      empty_context: Context::new(),
      clean: config.clean(),
      preserve: config.preserve(),
      verbose: config.verbose(),
      url_root: config.url_root(),
      url_post: config.url_post(),
      dir_resources: config.dir_resources(),
      dir_output: config.dir_output(),
      dir_templates: config.dir_templates(),
      dir_posts: config.dir_posts(),
      path_render: config.path_render()
    };

    // Get the absolute path of templates directory
    let t_d: String = Lib::path_absolute(&build.dir_templates);
    // Try to create the tera instance
    let mut tera: Tera = match Tera::parse(&t_d) {
      Ok(tera_object) => tera_object,
      Err(error) => panic!("Error: failed to render \"tera\" template:\n\n{}", error)
    };
    // Try to build the inheritance chains
    tera.build_inheritance_chains().expect("Error: failed to build tera inheritance chain");
    // Update values for `dir_templates` and `tera` in build
    build.dir_templates = t_d;
    build.tera = Some(tera);

    // Update value for `dir_resources`
    let d_r: String = Lib::path_absolute(&build.dir_resources);
    Lib::ensure_dir(&d_r);
    build.dir_resources = d_r;

    // Update value for `dir_output`
    let d_o: String = Lib::path_absolute(&build.dir_output);
    Lib::ensure_dir(&d_o);
    build.dir_output = d_o;

    // Ensure `dir_output` has been defined
    if build.dir_output.trim().len() < 1 {
      panic!("Error: Cannot clean build, `dir_output` has not been defined for this build instance.");
    }
    // Only clean if requested
    if build.clean == true {
      // Safe clean by default
      if build.preserve == true {
        // Log if verbose is enabled
        Lib::log(build.verbose, "yellow", "Clean", "folder", &build.dir_output);
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
        move_dir(&build.dir_output, path_move, &options).expect("Error: failed to move folder during clean");
        // Ensure the directory exists after move
        Lib::ensure_dir(&build.dir_output);
      }
      // Warning! This empties a directory by permanently deleting its contents!
      else {
        // Log if verbose is enabled
        Lib::log(build.verbose, "red", "Delete", "folder", &build.dir_output);
        // empty the directory
        Lib::empty_dir(&build.dir_output);
      }
    }

    // Return the new build instance
    build
  }
}
