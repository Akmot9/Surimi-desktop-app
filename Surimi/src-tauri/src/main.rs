use crate::file_integrity::{hash_file_list, list_files, write_json_file};

mod file_integrity ;
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, okok])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  
}

#[tauri::command]
fn greet(name: &str) -> String {
  println!("greet");
  format!("Hello, {}!", name)
}

#[tauri::command]
fn okok(name: &str) -> String {
  println!("okok");
  format!("okok, {}!", name)
}

