/* services */
use crate::services::apps;

/* models */
use crate::models::{
  response::Response,
  active_app::ActiveApp
};

#[tauri::command]
pub fn get_list_windows() -> String {
  let res: Response = apps::get_list_windows();
  format!("{}", serde_json::to_string(&res).unwrap())
}

#[tauri::command]
pub fn start_action_on_app(raw_list_active_apps: String) -> String {
  let list_active_apps: Vec<ActiveApp> = serde_json::from_str(&raw_list_active_apps).unwrap();
  let res: Response = apps::start_action_on_app(list_active_apps);
  format!("{}", serde_json::to_string(&res).unwrap())
}