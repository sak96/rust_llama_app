use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::Callback;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "dialog"])]
    async fn open(args: JsValue) -> JsValue;
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

pub fn prompt_path(callback: Callback<Option<String>>) {
    #[derive(Serialize)]
    struct Filter {
        name: &'static str,
        extensions: &'static [&'static str],
    }
    #[derive(Serialize)]
    struct OpenArgs {
        multiple: bool,
        filters: Vec<Filter>
    }
    spawn_local(async move {
        let filters = vec![Filter { extensions: &["bin"], name: "models" }];
        let args = to_value(&OpenArgs { filters, multiple: false }).unwrap();
        callback.emit(open(args).await.as_string())
    })
}

pub fn load_model(path: String, callback: Callback<bool>) {
    #[derive(Serialize)]
    struct LoadModelArgs {
        path: String,
    }
    spawn_local(async move {
        let args = to_value(&LoadModelArgs { path: path.clone() }).unwrap();
        callback.emit(invoke("load_model", args).await.is_truthy())
    })
}

pub fn model_status(callback: Callback<bool>) {
    spawn_local(async move {
        callback.emit(invoke("is_model_loaded", JsValue::NULL).await.is_truthy())
    })
}

pub fn unload_model() {
    spawn_local(async move {
        invoke("unload_model", JsValue::NULL).await;
    })
}
