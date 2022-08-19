use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConfigPosts {
  pub image: Option<String>,
  pub author: Option<String>,
  pub author_github: Option<String>
}

impl ConfigPosts {
  pub fn image(&self) -> &String {
    self.image.as_ref().unwrap()
  }

  pub fn author(&self) -> &String {
    self.author.as_ref().unwrap()
  }

  pub fn author_github(&self) -> &String {
    self.author_github.as_ref().unwrap()
  }
}

#[derive(Deserialize)]
pub struct Config {
  pub url_root: Option<String>,
  pub url_post: Option<String>,
  pub path_tera: Option<String>,
  pub path_render: Option<String>,
  pub dir_resources: Option<String>,
  pub dir_output: Option<String>,
  pub dir_posts: Option<String>,
  pub verbose: Option<bool>,
  pub clean: Option<bool>,
  pub preserve: Option<bool>,
  pub posts: ConfigPosts
}

impl Config {
  pub fn url_root(&self) -> &String {
    self.url_root.as_ref().unwrap()
  }

  pub fn url_post(&self) -> &String {
    self.url_post.as_ref().unwrap()
  }

  pub fn path_tera(&self) -> &String {
    self.path_tera.as_ref().unwrap()
  }

  pub fn path_render(&self) -> &String {
    self.path_render.as_ref().unwrap()
  }

  pub fn dir_resources(&self) -> &String {
    self.dir_resources.as_ref().unwrap()
  }

  pub fn dir_output(&self) -> &String {
    self.dir_output.as_ref().unwrap()
  }

  pub fn dir_posts(&self) -> &String {
    self.dir_posts.as_ref().unwrap()
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