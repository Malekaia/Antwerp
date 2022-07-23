use crate::antwerp::{Config, Lib};
use chrono::{Timelike, DateTime, Datelike};
use chrono::prelude::Local;
use fs_extra::dir::{move_dir, CopyOptions};

/// **Description**:
///
/// Empty the root folder (see `lib::empty_dir`)
pub fn clean_build(config: &Config) {
  // Safe clean by default
  if config.safe_clean == true {
    // Log if verbose is enabled
    Lib::log(config.verbose, "yellow", "Clean", "folder", &config.dir_output);
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
    move_dir(&config.dir_output, path_move, &options)
      .expect("Error: failed to move folder during clean");
    // Ensure the directory exists after move
    Lib::ensure_dir(&config.dir_output);
  }
  // Warning! This empties a directory by permanently deleting its contents!
  else {
    // Log if verbose is enabled
    Lib::log(config.verbose, "red", "Delete", "folder", &config.dir_output);
    // empty the directory
    Lib::empty_dir(&config.dir_output);
  }
}