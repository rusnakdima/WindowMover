/* sys lib */
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
  pub title: String,
  pub exe_file: String,
  pub hwnd: String,
  pub status: String,
}