//! Conveniently exposes Antwerp and its tools, allowing for the import of all structs, enums and functions provided by Antwerp under a single `use` declaration.
//!
//! This library uses common names for many of its functions. It's advisable to import functions under the `Antwerp` namespace to prevent clogging the current scope with common names such as `init`, `render` or `route`.
mod assets;
mod lib;
mod core;

pub use crate::antwerp::assets::{Asset, Post, Route, Template};
pub use crate::antwerp::core::Config;

#[allow(non_snake_case)]
pub mod Antwerp {
  pub use crate::antwerp::assets::*;
  pub use crate::antwerp::core::*;
}

#[allow(non_snake_case)]
pub mod Lib {
  pub use crate::antwerp::lib::*;
}