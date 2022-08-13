//! Conveniently exposes Antwerp and its tools, allowing for the import of all structs, enums and functions provided by Antwerp under a single `use` declaration.
//!
//! This library uses common names for many of its functions. It's advisable to import functions under the `Antwerp` namespace to prevent clogging the current scope with common names such as `build`, `render` or `route`.
//!
//! Provides functions that prevent code duplication - and standardises the implementation of frequently used tools in a fail-safe way, providing detailed error handling if not possible.
mod handlers;
mod helpers;
mod core;

// Expose Antwerp utilities using wrappers
pub use crate::handlers::{Asset, Post, Route, Routes, Template};
pub use crate::core::Config;

#[allow(non_snake_case)]
pub mod Antwerp {
  pub use crate::handlers::{Asset, assets, Post, Route, Routes, Template, tera, render, render_string, route, route_group};
  pub use crate::core::{clean_build, Config, build};
}

#[allow(non_snake_case)]
pub mod Lib {
  pub use crate::helpers::{exists, read_file, write_file, copy_file, ensure_dir, empty_dir, walk_dir};
  pub use crate::helpers::log;
  pub mod path {
    pub use crate::helpers::{join, from_cwd, absolute};
  }
  pub use crate::helpers::{string_to_slug, escape_html};
}