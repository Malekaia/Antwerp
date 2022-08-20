use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct ConfigPosts {
  pub image: Option<String>
}

impl ConfigPosts {
  pub fn image(&self) -> String {
    self.image.as_ref().unwrap().to_string()
  }
}