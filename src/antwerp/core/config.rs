use crate::antwerp::{Antwerp, Asset, Post, Route, Routes};
use tera::Tera;

pub struct Config<'a> {
  pub tera: Tera,
  pub url_root: &'a str,
  pub url_post: &'a str,
  pub dir_resources: String,
  pub dir_output: String,
  pub dir_posts: String,
  pub path_render: &'a str,
  pub clean: bool,
  pub verbose: bool,
  pub post_list: Vec<Post>,
  pub assets: Vec<Asset<'a>>,
  pub routes: Vec<Route<'a>>,
  pub route_groups: Vec<Routes<'a>>
}

impl Config<'_> {
  pub fn new(template_directory: String) -> Config<'static> {
    Config {
      tera: Antwerp::tera(template_directory),
      url_root: "",
      url_post: "",
      dir_resources: String::new(),
      dir_output: String::new(),
      dir_posts: String::new(),
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