use serde::Deserialize;
use crate::core::config::unwrap_string;

#[derive(Clone, Deserialize)]
pub struct ConfigHeader {
  pub image: Option<String>,
  pub credits: Option<String>
}

impl ConfigHeader {
  pub fn image(&self) -> String {
    unwrap_string(self.image.as_ref())
  }

  pub fn credits(&self) -> String {
    unwrap_string(self.credits.as_ref())
  }
}
