use crate::antwerp::{Config, Lib};

/// **Description**:
///
/// Empty the root folder (see `lib::empty_dir`)
pub fn clean_build(config: &Config) {
  // Log if verbose is enabled
  Lib::log(config.verbose, "red", "Empty", "folder", &config.dir_output);
  // Empty the directory
  Lib::empty_dir(&config.dir_output);
}