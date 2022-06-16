use crate::{lib::api::use_api, Route};
use common::ChannelList;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChannelListProps {
  pub server: String,
}

#[function_component(ChannelsSidebar)]
pub fn channels_sidebar(props: &ChannelListProps) -> Html {
  let channels: ChannelList = use_api("dms".to_string(), ()).unwrap_or(ChannelList {
    server: "".to_string(),
    channels: vec![],
  });

  html! {
    <div class={classes!(
      "flex",
      "flex-col",
      "h-screen",
      "bg-slate-800"
    )}>
      <div class={classes!(
        "flex",
        "items-center",
        "h-12",
        "p-2",
        "border-slate-700",
        "border-b-2",
        "font-bold"
      )}>
        <ChannelLink
          channel_server={ "@me".to_string() }
          channel={ "@friends".to_string() }
          display_name={ "Friends".to_string() }
        />
      </div>
      <nav class={classes!(
        "p-2"
      )}>
        { for channels.channels.iter().map(|channel| {
          html! {
            <ChannelLink
              channel_server={ props.server.clone() }
              channel={ format!("{}", channel.id) }
              display_name={ format!("{}", channel.id) }
            />
          }
        }) }
      </nav>
    </div>
  }
}

#[derive(Properties, PartialEq)]
struct ChannelLinkProps {
  channel_server: String,
  channel: String,
  display_name: String,
}

#[function_component(ChannelLink)]
fn channel_link(props: &ChannelLinkProps) -> Html {
  html! {
    <Link<Route> to={
      if props.channel_server == "@me" {
        Route::DirectChannel { id: props.channel.clone() }
      } else {
        Route::Channel {
          server: props.channel_server.clone(),
          id: props.channel.clone()
        }
      }
    }>
      <div class={classes!(
        "hover:bg-slate-700",
        "w-40",
        "px-2",
        "py-1",
        "rounded-xl",
        "text-left"
      )}>
        { &props.display_name }
      </div>
    </Link<Route>>
  }
}
