use crate::antwerp::{Config, Lib};
use regex::Regex;

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
  // source, check, overwrite
  Folder(&'a str, &'a str, bool),
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
pub fn assets(config: &Config) {
  for asset in &config.assets {
    // Handle different asset types
    match asset {
      // Copy static assets
      Asset::File(source, destination, overwrite) => {
        // Create the from and to paths
        let from: &str = &Lib::path::join(&config.dir_resources, source);
        let to: &str = &Lib::path::join(&config.dir_output, destination);
        // Log the update
        let overwrite_status: &str = if *overwrite == true { "overwrite: false" } else { "overwrite: true" };
        Lib::log(config.verbose, "blue", "Copy", overwrite_status, destination);
        // Copy the file to the intended destination
        Lib::copy_file(from, to, *overwrite);
      },

      // Copy (recursive) directories
      Asset::Folder(source, check, overwrite) => {
        // Regular expression used to check the matches
        let re_check: Regex = Regex::new(check).expect(&format!("Error: failed to create regex: {}", check));
        // Create required paths
        let search_path: &str = &Lib::path::join(&config.dir_resources, source);
        // Iterate the file paths
        for path_string in Lib::walk_dir(search_path) {
          // Ignore the match if it doesn't pass the check
          if !re_check.is_match(&path_string) {
            continue;
          }
          // Create the copy to and from paths
          let from: &str = &Lib::path::from_cwd(&path_string);
          let to: &str = &from.replace(&config.dir_resources, &config.dir_output);
          // Log the update
          let overwrite_status: &str = if *overwrite == true { "overwrite: false" } else { "overwrite: true" };
          let mut to_display: &str = &to.replace(&config.dir_output, "");
          to_display = if to_display.starts_with("/") { &to_display[1..to_display.len()] } else { to_display };
          Lib::log(config.verbose, "blue", "Copy", overwrite_status, to_display);
          // Copy the file
          Lib::copy_file(from, to, *overwrite);
        }
      },

      // Compile SCSS assets
      Asset::Scss(source, destination) => {
        // Create the from and to paths
        let from: &str = &Lib::path::join(&config.dir_resources, source);
        let to: &str = &Lib::path::join(&config.dir_output, destination);
        // Try to compile the SCSS stylesheet (return if possible, panic if not)
        match grass::from_path(from, &grass::Options::default()) {
          // Write the file if successful
          Ok(result) => {
            // Log the update
            Lib::log(config.verbose, "magenta", "Compile", "SCSS", destination);
            // Write the compiled SCSS to the destination file
            Lib::write_file(to, result);
          },
          // Panic and exit
          Err(error) => panic!("Error: failed to compile SASS stylesheets\n\n{:?}", error)
        };
      }
    }
  }
}