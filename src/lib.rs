//! ### Antwerp
//! [Antwerp](https://crates.io/crates/antwerp) is an open-source framework ported from JavaScript to Rust for GitHub pages and built with the Marcus HTML to MarkDown parser.
//! It outputs static web pages in `dist/` using HTML and MarkDown templates in `public/`, which are converted to HTML using the [Marcus](https://crates.io/crates/marcus) MarkDown to HTML parser.
//!
//! References & Getting Started:
//! - <https://crates.io/crates/antwerp>
//! - <https://github.com/Malekaia/Antwerp>
//! - <https://docs.rs/antwerp/latest/antwerp/>
//! - <https://crates.io/crates/marcus>
//! - <https://github.com/Malekaia/Marcus/>
//! - <https://docs.rs/marcus/latest/marcus/>
//! - <https://developer.mozilla.org/en-US/docs/Web/HTML>
//! - <https://www.markdownguide.org/>

mod filters;
mod parser;
use crate::filters::filter_output;
use crate::parser::parse_templates;
use std::collections::HashMap;
use std::fs::{create_dir_all, write};

/// Type alias representing a `Vector` of `String` for filter lists in block declarations.
///
/// For example, the following block:
///
/// `{% block ... filter_1 | filter_2 | filter_n %}...{% endblock ... %}`
///
/// Would produce a `Vector` equivalent to:
///
/// `vec![String::from("filter_1"), String::from("filter_2"), String::from("filter_n")]`
pub type Filters = Vec<String>;

/// Contains information for blocks derived from block declarations in templates.
///
/// Fields:
/// - `filters`: a `Vector` of `String` for filter lists in block declarations (see [`Filters`])
/// - `content_outer`: a String containing the outer content of a block (see `content_outer` of [`Block`])
/// - `content`: a String containing the inner content of a block (see `content` of [`Block`])
#[derive(Debug)]
pub struct Block {
  /// A `Vector` of `String` for filter lists in block declarations (see [Filters])
  pub filters: Filters,
  /// A String containing the outer content of a block.
  ///
  /// For example, an instance of `Block` derived from the following template:
  ///
  /// `{% block test filter_1 | filter_2 | filter_n %}This is a test block{% endblock test %}`
  ///
  /// Would have a `content_outer` field equal to:
  ///
  /// `String::from("{% block test filter_1 | filter_2 | filter_n %}This is a test block{% endblock test %}")`
  pub content_outer: String,
  /// A String containing the inner content of a block.
  ///
  /// For example, an instance of `Block` derived from the following template:
  ///
  /// `{% block test filter_1 | filter_2 | filter_n %}This is a test block{% endblock test %}`
  ///
  /// Would have a `content` field equal to:
  ///
  /// `String::from("This is a test block")`
  pub content: String
}

/// Type alias representing a `HashMap` where key: `String` and value: [`Block`].
///
/// A `HashMap` with this type is used to contain the blocks (organised by name) of a given HTML or MarkDown template.
///
/// For example, a template with a single block declaration:
///
/// `{% block test filter_1 | filter_2 | filter_n %}This is a test block{% endblock test %}`
///
/// Would produce the following `HashMap`:
///
/// ```rust
/// Blocks {
///   "test": Block {
///     filters: vec!["filter_1", "filter_2", "filter_n"],
///     content_outer: "{% block test filter_1 | filter_2 | filter_n %}This is a test block{% endblock test %}",
///     content: "This is a test block"
///   }
/// }
/// ```
pub type Blocks = HashMap<String, Block>;

/// Contains template information derived from HTML and MarkDown templates.
///
/// Fields:
/// - `extends`: a `String` containing the file path provided by an extends statement (see `extends` of [`Template`])
/// - `output`: a `String` containing the file path of the output file
/// - `output_dir`: a `String` containing the directory of the output file
/// - `content`: a `String` containing the file content of the given template
/// - `blocks`: contains the blocks (organised by name) of a given HTML or MarkDown template (see [`Blocks`])
#[derive(Debug)]
pub struct Template {
  /// A `String` containing the file path provided by an extends statement
  ///
  /// For example, an instance of `Template` for a file with the following extends statement:
  ///
  /// `{% extends "base.html" %}`
  ///
  /// Would have an `extends` field with a value equal to:
  ///
  /// `String::from("/home/<USERNAME>/<PATH>/<TO>/<PROJECT>/public/base.html")`
  ///
  /// **Note**: Absolute paths are obtained using the [`project-root`](https://crates.io/crates/project-root) crate, which finds the root directory of a project, relative to the location of the nearest [`Cargo.lock`](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html).
  ///
  /// **Note**: Paths are handled using [`Path`](https://doc.rust-lang.org/stable/std/path/struct.Path.html) and [`PathBuf`](https://doc.rust-lang.org/std/path/struct.PathBuf.html) and support Unix and Windows paths.
  pub extends: String,
  /// A `String` containing the file path of the output file
  pub output: String,
  /// A `String` containing the directory of the output file
  pub output_dir: String,
  /// A `String` containing the file content of the given template
  pub content: String,
  /// Contains the blocks (organised by name) of a given HTML or MarkDown template (see [`Blocks`])
  pub blocks: Blocks
}

/// Type alias representing a `HashMap` where key: `String` and value: [`Template`].
///
/// A `HashMap` with this type is used to contain the template information (organised by file path) of a given HTML or MarkDown template.
///
/// For example, the following Markdown template containing an extends statement and a single block:
///
/// ```markdown
///   {% extends "base.html" %}
///   {% block title %}Homepage{% endblock title %}
/// ```
///
/// Would produce the a `HashMap`similar to:
///
/// ```rust
/// Templates {
///   "/home/<USERNAME>/<PATH>/<TO>/<PROJECT>/public/file.md": Template {
///     extends: "/home/<USERNAME>/<PATH>/<TO>/<PROJECT>/public/base.html",
///     output: "/home/<USERNAME>/<PATH>/<TO>/<PROJECT>/dist/file.html",
///     output_dir: "/home/<USERNAME>/<PATH>/<TO>/<PROJECT>/dist/",
///     content: "{% extends "base.html" %}\n{% block title %}Homepage{% endblock title %}",
///     blocks: {
///       title: Block {
///         filters: vec![],
///         content_outer: "{% block title %}Homepage{% endblock title %}",
///         content: "Homepage"
///       }
///     }
///   }
/// }
/// ```
pub type Templates = HashMap<String, Template>;

/// Type aliases for filter methods
pub (crate) type FilterMethod = fn(String) -> String;
pub (crate) type FilterMethods = HashMap<String, FilterMethod>;

/// [`Build`](build) Parses HTML and MarkDown templates in `public/` and writes the HTML output to `dist/`
///
/// 1. Extracts template data from templates in `public/`
/// 3. Replaces block declarations in base template with content from child templates
/// 4. Inserts default block content for undefined blocks in child templates
/// 2. Uses filters to modify content before inserting into parent template
/// 5. Creates the output directory and writes the HTML template to the output file
///
/// **Note**: For sample templates and input, see [README.md](https://github.com/Malekaia/Antwerp#readme).
pub fn build() {
  // Get the parent templates
  let parent_templates: Templates = parse_templates("public/**/*.html");

  // Define output filter methods for later referencing (prevents multiple defines in loop and use of `lazy_static`)
  let mut filter_methods: FilterMethods = HashMap::new();
  // Trim the output
  filter_methods.insert(String::from("trim"), | output: String | output.trim().to_string());
  // Parse MarkDown to HTML
  filter_methods.insert(String::from("html"), | output: String | marcus::to_string(output));
  // Return the raw text
  filter_methods.insert(String::from("text"), | output: String | output);

  // Iterate the child templates
  for (file_path, template) in parse_templates("public/**/*.md") {
    // Ensure the parent template exists
    if !parent_templates.contains_key(&template.extends) {
      panic!("TemplateError: unknown template \"{}\"", &template.extends);
    }

    // Get the base template
    let (parent_file_path, parent): (&String, &Template) = parent_templates.get_key_value(&template.extends).unwrap();

    // Create and replace the HTML string
    let mut html: String = parent.content.to_owned();

    // Iterate the blocks
    for (name, block) in &template.blocks {
      // Ensure the block exists in the base template
      if !parent.blocks.contains_key(name) {
        panic!("TemplateError: block \"{}\" in \"{}\" not defined in base template (\"{}\")", name, file_path, parent_file_path);
      }
      // Replace the block content with the given template
      let parent_block: &Block = parent.blocks.get(name).unwrap();
      // Use parent filters by default if no child filters are defined
      let filters: &Vec<String> = if !block.filters.is_empty() {
        &block.filters
      } else {
        &parent_block.filters
      };
      // Replace the HTML with the filtered output
      html = html.replace(&parent_block.content_outer, &filter_output(&filter_methods, filters, &block.content));
    }

    // Iterate the parent blocks
    for (_, block) in &parent.blocks {
      // Replace unspecified blocks with their default value
      if html.contains(&block.content_outer) {
        html = html.replace(&block.content_outer, &filter_output(&filter_methods, &block.filters, &block.content));
      }
    }

    // Create the output file's directory
    assert!(create_dir_all(&template.output_dir).is_ok(), "CreateDirAllError: failed to create directory \"{}\"", &template.output_dir);
    // Write the HTML to the output file
    assert!(write(&template.output, &html).is_ok(), "WriteError: write \"html\" to \"{}\"", &template.output);
  }
}

/// Test for GitHub Workflows (`.github/workflows/build.yaml`)
#[cfg(test)]
mod tests {
  use crate as antwerp;
  #[test]
  fn antwerp() {
    antwerp::build();
  }
}
