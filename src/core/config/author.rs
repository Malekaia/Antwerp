use serde::Deserialize;
use crate::core::config::unwrap_string;

#[derive(Clone, Deserialize)]
pub struct ConfigAuthor {
  pub name: Option<String>,
  pub image: Option<String>,
  pub github_url: Option<String>,
  pub github_username: Option<String>
}

impl ConfigAuthor {
  pub fn name(&self) -> String {
    unwrap_string(self.name.as_ref())
  }

  pub fn image(&self) -> String {
    unwrap_string(self.image.as_ref())
  }

  pub fn github_url(&self) -> String {
    unwrap_string(self.github_url.as_ref())
  }

  pub fn github_username(&self) -> String {
    unwrap_string(self.github_username.as_ref())
  }
}
