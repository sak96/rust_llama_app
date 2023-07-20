use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
#[allow(non_snake_case)]
pub struct ChatProps {
    pub class: Classes,
    pub content: String,
}

#[function_component(Chat)]
pub fn chat(ChatProps { class, content }: &ChatProps) -> Html {
    html! { <div class={class.clone()} ~innerText={content.clone()}/> }
}
