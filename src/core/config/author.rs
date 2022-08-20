use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct ConfigAuthor {
  pub name: Option<String>,
  pub image: Option<String>,
  pub github_url: Option<String>,
  pub github_username: Option<String>
}

impl ConfigAuthor {
  pub fn name(&self) -> String {
    self.name.as_ref().unwrap().to_string()
  }

  pub fn image(&self) -> String {
    self.image.as_ref().unwrap().to_string()
  }

  pub fn github_url(&self) -> String {
    self.github_url.as_ref().unwrap().to_string()
  }

  pub fn github_username(&self) -> String {
    self.github_username.as_ref().unwrap().to_string()
  }
}