use crate::types::{VALID_FILTERS, Filters};
use marcus;

pub fn output(filters: &Filters, text: &String) -> String {
  // Prevent contradicting filters
  if filters.contains(&"text".to_string()) && filters.contains(&"html".to_string()) {
    panic!("TemplateError: contradicting filters, cannot output HTML and raw text simultaneously");
  }

  // Create an owned copy of the text `
  let mut output: String = text.to_owned();

  // Convert MarkDown to HTML by default
  if filters.is_empty() {
    return marcus::to_string(output);
  }

  // Iterate the filters
  for filter in filters {
    // Ensure the filter is valid
    if !VALID_FILTERS.contains(&filter.as_str()) {
      panic!("TemplateError: unknown filter \"{}\"", filter);
    }

    // Trim the string
    if filter == "trim" {
      output = output.trim().to_string();
    }
    // Parse MarkDown to HTML
    else if filter == "html" {
      output = marcus::to_string(output);
    }
  }

  // Return the string as is (including for `text`) filters
  output
}
