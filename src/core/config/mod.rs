mod author;
mod posts;

pub use crate::core::config::author::ConfigAuthor;
pub use crate::core::config::posts::ConfigHeader;
use serde::Deserialize;

pub fn unwrap_string(value: Option<&String>) -> String {
  match value {
    None => String::new(),
    Some(result) => result.to_owned()
  }
}

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
  pub header: ConfigHeader,
  pub author: ConfigAuthor
}

impl Config {
  pub fn url_root(&self) -> String {
    unwrap_string(self.url_root.as_ref())
  }

  pub fn url_post(&self) -> String {
    unwrap_string(self.url_post.as_ref())
  }

  pub fn path_render(&self) -> String {
    unwrap_string(self.path_render.as_ref())
  }

  pub fn dir_resources(&self) -> String {
    unwrap_string(self.dir_resources.as_ref())
  }

  pub fn dir_output(&self) -> String {
    unwrap_string(self.dir_output.as_ref())
  }

  pub fn dir_templates(&self) -> String {
    unwrap_string(self.dir_templates.as_ref())
  }

  pub fn dir_posts(&self) -> String {
    unwrap_string(self.dir_posts.as_ref())
  }

  pub fn verbose(&self) -> bool {
    match self.verbose {
      None => true,
      Some(result) => result
    }
  }

  pub fn clean(&self) -> bool {
    match self.clean {
      None => true,
      Some(result) => result
    }
  }

  pub fn preserve(&self) -> bool {
    match self.preserve {
      None => true,
      Some(result) => result
    }
  }
}
