use crate::api::{load_model, prompt_path};
use yew::prelude::*;

mod chat;
use chat::ChatWindow;

enum AppState {
    New,
    Loading,
    Chat,
}

#[function_component(App)]
pub fn app() -> Html {
    let app_state = use_state(|| AppState::New);
    let load_model = {
        let app_state = app_state.clone();
        Callback::from(move |_| {
            let app_state = app_state.clone();
            prompt_path(Callback::from(move |path| {
                let app_state = app_state.clone();
                if let Some(path) = path {
                    app_state.set(AppState::Loading);
                    load_model(
                        path,
                        Callback::from(move |done| {
                            if done {
                                app_state.set(AppState::Chat)
                            } else {
                                app_state.set(AppState::New)
                            }
                        }),
                    );
                }
            }))
        })
    };

    match *app_state {
        AppState::New => html! {
            <>
                <h3 ~innerText="Select Model" />
                <p>
                    {"You can download one of the models, from "}
                    <a href="https://huggingface.co/TheBloke/Wizard-Vicuna-7B-Uncensored-GGML" ~innerText="here"/>
                </p>
                <button onclick={load_model}>{"Load Model"}</button>
            </>
        },
        AppState::Loading => html! {
            <>
                <h3>{"Loading Model"}</h3>
                <progress /></>
        },
        AppState::Chat => html! { <ChatWindow />},
    }
}
