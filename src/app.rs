use crate::api::reply;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let greet_input_ref = use_node_ref();
    let name = use_state(String::new);
    let greet_msg = use_state(String::new);
    {
        let greet_msg = greet_msg.clone();
        let name = name.clone();
        let name2 = name.clone();
        use_effect_with_deps(
            move |_| {
                reply(
                    (*name).clone(),
                    Callback::from(move |input: String| greet_msg.set(input)),
                )
            },
            name2,
        );
    }
    let greet = {
        let name = name.clone();
        let greet_input_ref = greet_input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            name.set(
                greet_input_ref
                    .cast::<web_sys::HtmlInputElement>()
                    .unwrap()
                    .value(),
            );
        })
    };

    html! {
        <main class="container">
            <form class="row" onsubmit={greet}>
                <input id="greet-input" ref={greet_input_ref} placeholder="Enter a name..." />
                <button type="submit">{"Greet"}</button>
            </form>

            <p><b>{ &*greet_msg }</b></p>
        </main>
    }
}
