//! Exposes Antwerp's main `Config` struct and init method
mod config;
mod init;

pub use crate::antwerp::core::config::Config;
pub use crate::antwerp::core::init::init;