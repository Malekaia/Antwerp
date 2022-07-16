//! Exposes Antwerp structs, enums and utilities for convenient use
mod core;
mod lib;
mod posts;

pub use crate::antwerp::core::{Asset, Template};
pub use crate::antwerp::posts::{Post, PostsConfig};

#[allow(non_snake_case)]
pub mod Antwerp {
  pub use crate::antwerp::core::*;
  pub use crate::antwerp::posts::*;
}

#[allow(non_snake_case)]
pub mod Lib {
  pub use crate::antwerp::lib::*;
}