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
        Callback::from(move |_| {
            let chat_box = input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
            let user_reply = chat_box.value();
            chat_box.set_value("");
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
        max-width: 80%;
        margin-top: 10px;
        margin-bottom: 10px;
        border-radius: 10px;
        padding: 5px;
        "#
    );
    let user_chat = use_style!(
        r#"
        background-color: #358cf6;
        align-self: self-end;
        border-bottom-right-radius: 0px;
        "#
    );
    let ai_chat = use_style!(
        r#"
        color: #000000;
        background-color: #e2e2eb;
        align-self: self-start;
        border-bottom-left-radius: 0px;
        "#
    );
    let chat = use_style!(
        r#"
        display: flex;
        flex-grow: 1;
        flex-direction: column;
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
            <div class="chat-box">
                <input ref={input_ref} placeholder="Enter chat message..." />
                <button onclick={reply_chat}>{"Send"}</button>
            </div>
        </main>
    }
}
