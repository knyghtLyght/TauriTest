#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)] // Macro that disables the command prompt window that would normally pop up on Windows if you run a bundled app

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");


}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("hello, {}!", name)
}