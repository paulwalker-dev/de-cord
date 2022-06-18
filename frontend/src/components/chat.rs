use crate::{
  components::friend_list::FriendList,
  lib::api::{post_api, use_api},
};
use common::{Channel, Message, UserProfile};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct ChatProps {
  pub server: String,
  pub channel: String,
}

#[function_component(Chat)]
pub fn chat(props: &ChatProps) -> Html {
  let channel: Channel = use_api(
    format!("channel/{}/{}", &props.server, &props.channel),
    vec![props.channel.clone()],
  )
  .unwrap_or(Channel {
    id: 0,
    user_ids: vec![],
    messages: vec![],
  });

  let message_ref = use_node_ref();
  let onsubmit = {
    let message_ref = message_ref.clone();
    let props = props.clone();
    move |e: FocusEvent| {
      e.prevent_default();
      if let Some(input) = message_ref.cast::<HtmlInputElement>() {
        post_api(
          &format!("channel/{}/{}", &props.server, &props.channel),
          &Message {
            id: 100,
            user_id: 0,
            contents: input.value(),
          },
        )
        .expect("Working server");
      }
    }
  };

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
            <div class={classes!(
              "flex-1",
              "flex",
              "flex-col",
              "p-3"
            )}>
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
              <form
                class={classes!(
                  "flex"
                )}
                onsubmit={ onsubmit }
              >
                <input
                  class={classes!(
                    "flex-1"
                  )}
                  ref={ message_ref }
                />
              </form>
            </div>
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
