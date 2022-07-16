use crate::antwerp::Lib;

/// **Description**:
///
/// Empty the root folder (see `lib::empty_dir`)
pub fn empty_root(root: &str) {
  // Log the update
  Lib::print("red", "Empty", "folder", root);
  // Empty the directory
  Lib::empty_dir(root);
}