use crate::{components::friend_list::FriendList, lib::api::use_api};
use common::{Channel, Message, UserProfile};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChatProps {
  pub channel: String,
}

#[function_component(Chat)]
pub fn chat(props: &ChatProps) -> Html {
  let channel: Channel = use_api(
    format!("dms/{}", &props.channel),
    vec![props.channel.clone()],
  )
  .unwrap_or(Channel {
    id: 0,
    user_ids: vec![],
    messages: vec![],
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
          { &props.channel }
        </p>
      </div>
      {
        if props.channel != "@friends" {
          html! {
            <main class={classes!(
              "flex-1",
              "flex",
              "flex-col-reverse",
              "p-3"
            )}>
              { for channel.messages.iter().map(|message| {
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
  let user: UserProfile = use_api(
    format!("profile/{}", props.message.user_id),
    vec![props.message.clone()],
  )
  .unwrap_or(UserProfile {
    id: 0,
    username: "".to_string(),
  });

  html! {
    <div>
      <p class={classes!(
        "font-bold"
      )}>
        { &user.username }
      </p>
      <p>
        { &props.message.contents }
      </p>
    </div>
  }
}
