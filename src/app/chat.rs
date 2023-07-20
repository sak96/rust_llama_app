use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
#[allow(non_snake_case)]
pub struct ChatProps {
    pub isUser: bool,
    pub content: String,
}

#[function_component(Chat)]
pub fn chat(
    ChatProps {
        isUser: is_user,
        content,
    }: &ChatProps,
) -> Html {
    let class = if *is_user { "is-user" } else { "is-ai" };
    html! {
        <div class={classes!(class)}>{content}</div>
    }
}
