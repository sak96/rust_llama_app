use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}


pub fn reply(message: String, callback: Callback<String>) {
    #[derive(Serialize)]
    struct ReplyArgs {
        message: String,
    }
    spawn_local(async move {
        let args = to_value(&ReplyArgs { message }).unwrap();
        let new_msg = invoke("reply", args).await.as_string().unwrap();
        callback.emit(new_msg);
    });
}
