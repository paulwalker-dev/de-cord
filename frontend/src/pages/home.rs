use crate::components::{channels_sidebar::ChannelsSidebar, chat::Chat};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
    pub server: String,
    pub channel: String,
}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
    html! {
      <div class={classes!(
        "flex-1",
        "flex",
        "flex-row"
      )}>
        <ChannelsSidebar server={ "@me" } />
        <Chat
      server={ props.server.clone() }
      channel={ props.channel.clone() }
    />
      </div>
    }
}
