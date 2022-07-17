use tera::Tera;
use crate::antwerp::{Antwerp, Asset, Post, Route};

pub struct Config<'a> {
  pub tera: Tera,
  pub uri: &'a str,
  pub uri_post: &'a str,
  pub dir_dist: &'a str,
  pub dir_templates: &'a str,
  pub path_render: &'a str,
  pub clean: bool,
  pub verbose: bool,
  pub post_list: Vec<Post>,
  pub assets: Vec<Asset<'a>>,
  pub routes: Vec<Route<'a>>,
  pub route_groups: Vec<Route<'a>>
}

impl Config<'_> {
  pub fn new(template_directory: &str) -> Config<'static> {
    Config {
      tera: Antwerp::tera(template_directory),
      uri: "",
      uri_post: "",
      dir_dist: "",
      dir_templates: "",
      path_render: "",
      clean: false,
      verbose: false,
      post_list: vec![],
      assets: vec![],
      routes: vec![],
      route_groups: vec![],
    }
  }
}