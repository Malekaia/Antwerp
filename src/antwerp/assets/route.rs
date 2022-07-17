use tera::Context;
use crate::antwerp::Template;

pub enum Route<'a> {
  Page(&'a str, &'a str, Context),
  Group(&'a str, Vec<Template<'a>>)
}