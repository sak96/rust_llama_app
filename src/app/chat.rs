use stylist::yew::use_style;
use yew::prelude::*;

use crate::api::reply;

#[allow(non_camel_case_types)]
#[derive(Properties, Clone, PartialEq)]
pub struct ChatWindowProps {
    pub closed: Callback<()>,
}

#[function_component(ChatWindow)]
pub fn chat_window(ChatWindowProps { closed }: &ChatWindowProps) -> Html {
    let input_ref = use_node_ref();
    let is_replying = use_state(|| false);
    let chats = use_mut_ref(Vec::new);

    let reply_chat = {
        let chats = chats.clone();
        let input_ref = input_ref.clone();
        let is_replying = is_replying.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let chat_box = input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
            let chat_box_content = chat_box.inner_text();
            let user_reply = chat_box_content.trim().to_string();
            if !user_reply.is_empty() {
                chat_box.set_inner_text("");
                chats.borrow_mut().push((true, user_reply.clone()));
                is_replying.set(true);
                reply(user_reply, {
                    let chats = chats.clone();
                    let is_replying = is_replying.clone();
                    Callback::from(move |input: String| {
                        chats
                            .clone()
                            .borrow_mut()
                            .push((false, input.trim().to_string()));
                        is_replying.set(false);
                    })
                })
            }
        })
    };

    let on_close = {
        let closed = closed.clone();
        Callback::from(move |_| closed.emit(()))
    };

    let chat_bubble = use_style!(
        r#"
        margin: 0.5rem;
        max-width: 80%;
        padding: 0.5rem;
        border-radius: 1rem;
        "#
    );
    let user_chat = use_style!(
        r#"
        background-color: DodgerBlue;
        align-self: self-end;
        border-bottom-right-radius: 0;
        "#
    );
    let ai_chat = use_style!(
        r#"
        color: Black;
        background-color: GhostWhite;
        align-self: self-start;
        border-bottom-left-radius: 0;
        "#
    );
    let chat = use_style!(
        r#"
        display: flex;
        flex-grow: 1;
        flex-direction: column;
        height: 100vh;
        "#
    );
    let chat_box = use_style!(
        r#"
            display: flex;
            border: 1px solid DodgerBlue;
            border-radius: 1rem;
            margin: 0.5rem;
            justify-self: end;
            flex-direction: row;
            div {
                outline: none;
                border: none;
                border-radius: 0.5rem;
                background: none;
                padding: 1rem;
                flex: 1;
            }
            div:empty:before {
                content: attr(data-placeholder);
            }
            button {
                min-width: 10rem;
                border: none;
                border-radius: 1rem;
            }
            .submit-btn {
                background-color: DodgerBlue;
            }
            .close-btn {
                background-color: Tomato;
            }
        "#
    );

    html! {
        <main class={chat}>
            {
                chats.clone().borrow().iter().map(|(is_user, content)| {
                    let class = if *is_user {&user_chat} else {&ai_chat};
                    html! { <div class={classes!(chat_bubble.clone(),class.clone())} ~innerText={content.clone()}/> }
                }).collect::<Html>()
            }
            if *is_replying {
                <div class={classes!(ai_chat, chat_bubble)}>
                    {"Replying:"} <progress />
                </div>
            }
            <form class={chat_box} onsubmit={reply_chat}>
                <div
                    contenteditable="plaintext-only"
                    ref={input_ref}
                    data-placeholder="Enter chat message... Press Tab+Enter" />
                <button class="submit-btn" disabled={*is_replying}>{"Send"}</button>
                <button class="close-btn" onclick={on_close}>{"Close Chat"}</button>
            </form>
        </main>
    }
}
