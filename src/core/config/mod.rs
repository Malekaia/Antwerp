mod author;
mod posts;

use serde::Deserialize;
pub use crate::core::config::author::ConfigAuthor;
pub use crate::core::config::posts::ConfigPosts;

#[derive(Clone, Deserialize)]
pub struct Config {
  pub url_root: Option<String>,
  pub url_post: Option<String>,
  pub path_render: Option<String>,
  pub dir_resources: Option<String>,
  pub dir_output: Option<String>,
  pub dir_templates: Option<String>,
  pub dir_posts: Option<String>,
  pub verbose: Option<bool>,
  pub clean: Option<bool>,
  pub preserve: Option<bool>,
  pub posts: ConfigPosts,
  pub author: ConfigAuthor
}

impl Config {
  pub fn url_root(&self) -> String {
    self.url_root.as_ref().unwrap().to_string()
  }

  pub fn url_post(&self) -> String {
    self.url_post.as_ref().unwrap().to_string()
  }

  pub fn path_render(&self) -> String {
    self.path_render.as_ref().unwrap().to_string()
  }

  pub fn dir_resources(&self) -> String {
    self.dir_resources.as_ref().unwrap().to_string()
  }

  pub fn dir_output(&self) -> String {
    self.dir_output.as_ref().unwrap().to_string()
  }

  pub fn dir_templates(&self) -> String {
    self.dir_templates.as_ref().unwrap().to_string()
  }

  pub fn dir_posts(&self) -> String {
    self.dir_posts.as_ref().unwrap().to_string()
  }

  pub fn verbose(&self) -> bool {
    self.verbose.unwrap()
  }

  pub fn clean(&self) -> bool {
    self.clean.unwrap()
  }

  pub fn preserve(&self) -> bool {
    self.preserve.unwrap()
  }
}