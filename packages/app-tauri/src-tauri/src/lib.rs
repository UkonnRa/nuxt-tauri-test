use chrono::DateTime;

#[tauri::command]
async fn greet(name: String) -> String {
  let id = backend_core::create_payload().await;
  let (secs, nsecs) = id.get_timestamp().unwrap().to_unix();
  let timestamp = DateTime::from_timestamp(secs as i64, nsecs).unwrap();

  format!(
    "Hello, {}! You've been greeted from Rust, with ID = {} in Timestamp = {}!",
    name, id, timestamp
  )
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
