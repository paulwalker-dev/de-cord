use crate::components::friend_list::FriendList;
use common::{Message, User};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChatProps {
  pub id: String,
}

#[function_component(Chat)]
pub fn chat(props: &ChatProps) -> Html {
  let messages = use_state(|| {
    vec![Message {
      user: User {
        id: "nanoid!()".to_string(),
        username: "Test User".to_string(),
      },
      contents: "Hello, World!".to_string(),
    }]
  });

  html! {
    <div class={classes!(
      "flex",
      "flex-col",
      "flex-1"
    )}>
      <div class={classes!(
        "flex",
        "items-center",
        "h-12",
        "p-2",
        "border-slate-700",
        "border-b-2"
      )}>
        <p class={classes!(
          "font-bold"
        )}>
          { &props.id }
        </p>
      </div>
      {
        if props.id != "@friends" {
          html! {
            <main class={classes!(
              "flex-1",
              "flex",
              "flex-col-reverse",
              "p-3"
            )}>
              { for messages.iter().map(|message| {
                html! {
                  <ChatMessage message={message.clone()} />
                }
              }) }
            </main>
          }
        } else {
          html! {
            <FriendList />
          }
        }
      }
    </div>
  }
}

#[derive(Properties, PartialEq)]
struct ChatMessageProps {
  message: Message,
}

#[function_component(ChatMessage)]
fn chat_message(props: &ChatMessageProps) -> Html {
  html! {
    <div>
      <p class={classes!(
        "font-bold"
      )}>
        { &props.message.user.username }
      </p>
      <p>
        { &props.message.contents }
      </p>
    </div>
  }
}
