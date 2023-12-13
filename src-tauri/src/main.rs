// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod file_integrity ;

use crate::file_integrity::list_files; 



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_number_of_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn get_number_of_files()  {
    list_files(None).await.map_err(|e| e.to_string());
}