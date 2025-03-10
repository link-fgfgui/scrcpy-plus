use serde_json::{ json, Value };
mod system_command;
use system_command::{ scrcpy, adb };
use tauri::{WindowEvent,Window};

#[tauri::command]
fn environment() -> Value {
  return json!({
    "app_version":  env!("CARGO_PKG_VERSION"),
    "releases_url": "https://api.github.com/repos/Frontesque/scrcpy-plus/releases"
  });
}

#[tauri::command]
fn get_dependency_versions() -> Value {
  let scrcpy_version: String = scrcpy(vec!["--version"]).split_whitespace().nth(1).unwrap_or("Unable to find SCRCPY").to_string();
  let adb_version:    String = adb(vec!["--version"]).split_whitespace().nth(6).unwrap_or("Unable to find ADB").to_string();
  return json!({
    "scrcpy_plus":  env!("CARGO_PKG_VERSION"),
    "scrcpy":       scrcpy_version,
    "adb":          adb_version
  });
}

#[tauri::command]
fn exec_adb(args: Vec<&str>) -> String {
  return adb(args);
}

#[tauri::command]
fn exec_scrcpy(args: Vec<&str>) -> String {
  return scrcpy(args);
}

#[tauri::command]
fn run_scrcpy(args: Vec<&str>) {
  let args_copy: Vec<&str> = args.to_owned();
  scrcpy(args_copy);
}

// #[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .on_window_event(|_window: &Window, event: &WindowEvent| {
      if let WindowEvent::CloseRequested { .. } = event {
          adb(vec!["kill-server"]);
      }
    })
    .invoke_handler(tauri::generate_handler![get_dependency_versions, exec_adb, exec_scrcpy, run_scrcpy, environment])
    .run(tauri::generate_context!())
    .expect("error while running tauri application")
}
