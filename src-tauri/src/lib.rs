/* imports */
mod controllers;
mod services;
mod models;
mod helpers;

use controllers::about::{
    download_update,
    get_binary_name_file
};

use controllers::apps::{
    get_list_windows,
    start_action_on_app
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_binary_name_file,
            download_update,

            get_list_windows,
            start_action_on_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
