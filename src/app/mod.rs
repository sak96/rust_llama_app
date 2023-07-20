use crate::api::reply;
use stylist::yew::use_style;
use yew::prelude::*;

mod chat;
use chat::Chat;

#[function_component(App)]
pub fn app() -> Html {
    let input_ref = use_node_ref();
    let chat_trigger = use_force_update();
    let chats = use_mut_ref(Vec::new);

    let reply_chat = {
        let chats = chats.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let chat_box = input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
            let user_reply = chat_box.inner_text();
            chat_box.set_inner_text("");
            let chats = chats.clone();
            let chat_trigger = chat_trigger.clone();
            chats.clone().borrow_mut().push((true, user_reply.clone()));
            chat_trigger.force_update();
            reply(
                user_reply,
                Callback::from(move |input: String| {
                    chats.clone().borrow_mut().push((false, input));
                    chat_trigger.force_update()
                }),
            )
        })
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
            button {
                background-color: DodgerBlue;
                min-width: 10rem;
                border: none;
                border-radius: 1rem;
            }
        "#
    );

    html! {
        <main class={chat}>
            {
                chats.clone().borrow().iter().map(|(is_user, content)| {
                    let class = if *is_user {&user_chat} else {&ai_chat};
                    html!{<Chat class={classes!(chat_bubble.clone(),class.clone())} content={content.clone()}></Chat>}
                }).collect::<Html>()
            }
            <form class={chat_box} onsubmit={reply_chat}>
                <div contenteditable="plaintext-only" ref={input_ref} placeholder="Enter chat message..." />
                <button >{"Send"}</button>
            </form>
        </main>
    }
}
