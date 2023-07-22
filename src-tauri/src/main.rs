// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod bot;
use bot::ChatBot;
use tauri::{async_runtime::Mutex, State};


#[tauri::command]
async fn reply(message: &str, state: State<'_, Mutex<Option<ChatBot>>>) -> Result<String, &'static str> {
    let mut state = state.lock().await;
    Ok(state.as_mut().ok_or("failed to load model")?.get_reply(message))
}

#[tauri::command]
async fn load_model(path: &str, state: State<'_, Mutex<Option<ChatBot>>>) -> Result<bool, ()> {
    // TODO: allow new to fail.
    let model = ChatBot::new(&path.into());
    let mut state = state.lock().await;
    state.replace(model);
    Ok(true)
}

#[tauri::command]
async fn is_model_loaded(state: State<'_, Mutex<Option<ChatBot>>>) -> Result<bool, ()> {
    Ok(state.lock().await.is_some())
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(None as Option<ChatBot>))
        .invoke_handler(tauri::generate_handler![reply, load_model, is_model_loaded])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
