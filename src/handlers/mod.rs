//! Exposes enums and structs used to encapsulate site data & resources and functions used to copy, compile and render assets, stylesheets and templates.
mod assets;
mod posts;
mod templates;

pub use crate::handlers::assets::{Asset, assets};
pub use crate::handlers::posts::Post;
pub use crate::handlers::templates::{Route, Routes, Template, tera, render, render_string, route, route_group};