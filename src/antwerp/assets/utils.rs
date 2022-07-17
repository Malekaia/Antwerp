use crate::antwerp::{Config, Lib};

/// **Description**:
///
/// Empty the root folder (see `lib::empty_dir`)
pub fn empty_root(config: &Config, root: &str) {
  // Log if verbose is enabled
  Lib::log(config.verbose, "red", "Empty", "folder", root);
  // Empty the directory
  Lib::empty_dir(root);
}