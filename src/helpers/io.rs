use colored::Colorize;

/// **Description**:
///
/// Use `println` to output a colourised string about a certain action
pub fn log(verbose: bool, color: &str, action: &str, category: &str, target: &str) {
  if verbose == true {
    println!("{} ({}): {}", format!("{action}").color(color).bold(), format!("{category}").bold(), target);
  }
}
