#[macro_use]
extern crate lazy_static;
mod build;
mod fileio;
mod template;

fn main() {
  build::templates(template::list());
}
