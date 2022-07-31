//! Conveniently exposes Antwerp and its tools, allowing for the import of all structs, enums and functions provided by Antwerp under a single `use` declaration.
//!
//! This library uses common names for many of its functions. It's advisable to import functions under the `Antwerp` namespace to prevent clogging the current scope with common names such as `build`, `render` or `route`.
mod handlers;
mod lib;
mod core;

pub use crate::handlers::*;
pub use crate::core::*;

#[allow(non_snake_case)]
pub mod Antwerp {
  pub use crate::handlers::*;
  pub use crate::core::*;
}

#[allow(non_snake_case)]
pub mod Lib {
  pub use crate::lib::*;
}