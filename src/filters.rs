use crate::Filters;
use marcus;

// A list containing filter names and output methods
const FILTER_LIST: &[(&str, fn(String) -> String)] = &[
  // Return the raw text
  ("text", | output: String | output),
  // Trim the output
  ("trim", | output: String | output.trim().to_string()),
  // Parse MarkDown to HTML
  ("html", | output: String | marcus::to_string(output))
];

pub (crate) fn filter_output(filters: &Filters, text: &String) -> String {
  // Create an owned copy of the text `
  let mut output: String = text.to_owned();

  // Convert MarkDown to HTML by default
  if filters.is_empty() {
    return marcus::to_string(output);
  }

  // Iterate the user filters
  for filter in filters {
    let mut found: bool = false;
    // Iterate the `FILTER_LIST` const
    for (name, method) in FILTER_LIST {
      // Filter the output (if requested)
      if filter == name {
        found = true;
        output = method(output);
      }
    }
    // Validate filters
    if found == false {
      panic!("TemplateError: undefined filter (\"{}\")", filter);
    }
  }

  // Return the string as is (including for `text`) filters
  output
}
