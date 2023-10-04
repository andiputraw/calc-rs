
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn parse_number(expression : &str) -> Result<f64, String> {
    return exmex::eval_str::<f64>(expression).map_err(|err| err.msg().to_owned())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,parse_number])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
