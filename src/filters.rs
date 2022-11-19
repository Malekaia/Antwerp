use crate::{Filters, FilterMethods};
use marcus;





pub (crate) fn filter_output(filter_methods: &FilterMethods, filters: &Filters, text: &String) -> String {
  // Create an owned copy of the text `
  let mut output: String = text.to_owned();

  // Convert MarkDown to HTML by default
  if filters.is_empty() {
    return marcus::to_string(output);
  }

  // Iterate the user filters
  for filter in filters {
    // Validate each filter
    if !filter_methods.contains_key(filter) {
      panic!("TemplateError: undefined filter (\"{}\")", filter);
    }
    // Filter the output (if requested)
    output = filter_methods.get(filter).unwrap()(output);
  }

  // Return the string as is (including for `text`) filters
  output
}
