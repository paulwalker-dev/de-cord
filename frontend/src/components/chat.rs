use crate::{components::friend_list::FriendList, lib::api::get};
use common::{Channel, Message, UserProfile};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChatProps {
  pub server: String,
  pub channel: String,
}

#[function_component(Chat)]
pub fn chat(props: &ChatProps) -> Html {
  let channel_default = Channel {
    id: 0,
    user_ids: vec![],
    messages: vec![],
  };
  let channel = use_state(|| channel_default.clone());
  {
    let channel = channel.clone();
    let channel_name = props.channel.clone();
    use_effect_with_deps(
      move |_| {
        wasm_bindgen_futures::spawn_local(async move {
          let responce: Channel = get(&format!("dms/{}", channel_name))
            .await
            .unwrap_or(channel_default);
          channel.set(responce);
        });
        || ()
      },
      vec![props.server.clone(), props.channel.clone()],
    )
  }

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
  let user_default = UserProfile {
    id: 0,
    username: "".to_string(),
  };
  let user = use_state(|| user_default.clone());
  {
    let user = user.clone();
    let user_id = props.message.clone().user_id;
    use_effect_with_deps(
      move |_| {
        wasm_bindgen_futures::spawn_local(async move {
          let responce: UserProfile = get(&format!("profile/{}", user_id))
            .await
            .unwrap_or(user_default);
          user.set(responce);
        });
        || ()
      },
      (),
    )
  }

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
