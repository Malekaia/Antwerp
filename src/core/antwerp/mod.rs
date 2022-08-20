mod build;
mod config;

use crate::Config;
use tera::{Context, Tera};

pub struct Antwerp {
  pub config: Config,
  pub tera: Option<Tera>,
  pub empty_context: Context,
  pub clean: bool,
  pub preserve: bool,
  pub verbose: bool,
  pub url_root: String,
  pub url_post: String,
  pub dir_resources: String,
  pub dir_output: String,
  pub dir_templates: String,
  pub dir_posts: String,
  pub path_render: String
}