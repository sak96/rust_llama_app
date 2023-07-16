// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use llm::{models::Llama, InferenceSession, Model};
use tauri::{async_runtime::Mutex, State};

const MACHINE: &str = "### MACHINE ";
const HUMAN: &str = "### HUMAN";

struct StateModel {
    model: Arc<Llama>,
    session: InferenceSession,
}

#[tauri::command]
async fn reply(prompt: &str, state: State<'_, Mutex<StateModel>>) -> Result<String, ()> {
    let mut state = state.lock().await;
    let mut buf = String::new();
    // FIXME: cloning is not always good.
    let model = Arc::into_inner(state.model.clone()).unwrap();
    state
        .session
        .infer::<std::convert::Infallible>(
            &model,
            &mut rand::thread_rng(),
            &llm::InferenceRequest {
                prompt: &format!("{HUMAN}\n{prompt}\n{MACHINE}:"),
                parameters: Some(&llm::InferenceParameters::default()),
                play_back_previous_tokens: false,
                maximum_token_count: None,
            },
            &mut Default::default(),
            |token| {
                buf.push_str(token);

                Ok(())
            },
        )
        .unwrap_or_else(|e| panic!("{e}"));
    Ok(buf)
}

fn main() {
    // TODO: load this from UI
    let model_path = std::env::var("MODEL_PATH").expect("MODEL_PATH must be set");
    let model = Llama::load(
        &std::path::PathBuf::from(&model_path),
        Default::default(),
        llm::load_progress_callback_stdout,
    )
    .unwrap_or_else(|err| panic!("Failed to load model from {model_path:?}: {err}"));
    // TODO: allow renewing sessions.
    let session = model.start_session(Default::default());

    tauri::Builder::default()
        .manage(Mutex::new(StateModel {
            model: Arc::new(model),
            session,
        }))
        .invoke_handler(tauri::generate_handler![reply])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
