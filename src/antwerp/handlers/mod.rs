//! Exposes enums and structs used to encapsulate site data & resources and functions used to copy, compile and render assets, stylesheets and templates.
mod assets;
mod posts;
mod templates;
mod utils;

pub use crate::antwerp::handlers::assets::{Asset, assets};
pub use crate::antwerp::handlers::posts::Post;
pub use crate::antwerp::handlers::templates::{Template, Route, Routes, tera, render, render_string, route, route_group};
pub use crate::antwerp::handlers::utils::clean_build;