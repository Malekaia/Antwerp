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
pub fn assets(config: &Config) {
  for asset in &config.assets {
    // Handle different asset types
    match asset {
      // Copy static assets
      Asset::File(source, destination, overwrite) => {
        // Log the update
        let overwrite_status: &str = if *overwrite == true { "overwrite: false" } else { "overwrite: true" };
        Lib::log(config.verbose, "blue", "Copy", overwrite_status, &destination);

        // Copy the file to the intended destination
        Lib::copy_file(&source, &destination, *overwrite);
      },

      // Copy (recursive) directories
      Asset::Folder(source, mut destination, check, overwrite) => {
        // Regular expression used to remove the root of the source path
        let re_root: Regex = Regex::new(r"^(.*?)/(.*?)$").unwrap();
        // Regular expression used to check the matches
        let re_check: Regex = Regex::new(check).expect(&format!("Error: failed to create regex: {}", check));

        for path in Lib::walk_dir(source) {
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
          let overwrite_status: &str = if *overwrite == true { "overwrite: false" } else { "overwrite: true" };
          Lib::log(config.verbose, "blue", "Copy", overwrite_status, new_destination);
          // Copy the file
          Lib::copy_file(&path, new_destination, *overwrite);
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
        Lib::log(config.verbose, "magenta", "Compile", "SCSS", &destination);
        // Write the compiled SCSS to the destination file
        Lib::write_file(&destination, &result);
      }
    }
  }
}