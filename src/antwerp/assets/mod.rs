//! Exposes enums and structs used to encapsulate site data & resources and functions used to copy, compile and render assets, stylesheets and templates.
mod assets;
mod posts;
mod route;
mod templating;
mod utils;

pub use crate::antwerp::assets::assets::{Asset, assets};
pub use crate::antwerp::assets::posts::Post;
pub use crate::antwerp::assets::route::Route;
pub use crate::antwerp::assets::templating::{Template, tera, render, render_string, route, route_group};
pub use crate::antwerp::assets::utils::clean_build;