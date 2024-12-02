/* sys lib */
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveApp {
  pub hwnd: String,
  pub isMove: bool,
  pub isOnTop: bool
}
