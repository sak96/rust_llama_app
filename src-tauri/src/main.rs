// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bot;
use bot::ChatBot;
use tauri::{async_runtime::Mutex, State};

const MACHINE: &str = "### MACHINE ";
const HUMAN: &str = "### HUMAN";

#[tauri::command]
async fn reply(prompt: &str, state: State<'_, Mutex<ChatBot>>) -> Result<String, ()> {
    let mut state = state.lock().await;
    Ok(state.get_reply(prompt))
}

fn main() {
    // TODO: load this from UI
    let model_path = std::env::var("MODEL_PATH").expect("MODEL_PATH must be set");
    let bot = ChatBot::new(&model_path.into());
    tauri::Builder::default()
        .manage(Mutex::new(bot))
        .invoke_handler(tauri::generate_handler![reply])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
