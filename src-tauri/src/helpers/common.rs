/* models */
use crate::models::response::Response;

pub async fn get_binary_name_file() -> Response {
  let mut _name_app = String::new();
  if cfg!(target_os = "linux") {
    _name_app = "windowmover".to_string();
  } else if cfg!(target_os = "windows") {
    _name_app = "windowmover.exe".to_string();
  } else if cfg!(target_os = "macos") {
    _name_app = "windowmover.app".to_string();
  } else if cfg!(target_os = "android") {
    _name_app = "windowmover.apk".to_string();
  } else {
    return Response {
      status: "error".to_string(),
      message: "Unknown target platform".to_string(),
      data: "".to_string(),
    };
  }

  return Response {
      status: "success".to_string(),
      message: "".to_string(),
    data: _name_app,
  };
}