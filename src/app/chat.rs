use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ChatProps {
    pub is_user: bool,
    pub content: String,
}

#[function_component(Chat)]
pub fn chat(ChatProps { is_user, content }: &ChatProps) -> Html {
    html! {
        <div class={classes!(is_user.then_some("is-user"))}>{content}</div>
    }
}
