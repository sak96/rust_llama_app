use crate::api::reply;
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

    html! {
        <main class="container">
            {
                chats.clone().borrow().iter().map(|(is_user, content)| html!{
                    <Chat isUser={is_user} content={content.clone()}></Chat>
                }).collect::<Html>()
            }
            <div class="chat-box">
                <input ref={input_ref} placeholder="Enter chat message..." />
                <button onclick={reply_chat}>{"Send"}</button>
            </div>
        </main>
    }
}
