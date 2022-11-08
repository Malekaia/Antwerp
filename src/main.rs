#[macro_use]
extern crate lazy_static;
mod fileio;
mod template;

fn main() {
  println!("{:#?}", template::list());
}
