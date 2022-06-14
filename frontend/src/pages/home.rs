use crate::components::{channel_list::ChannelList, chat::Chat};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HomeProps {
  pub channel: String,
}

#[function_component(Home)]
pub fn home(props: &HomeProps) -> Html {
  html! {
    <div
      class={classes!(
        "flex-1",
        "flex",
        "flex-row"
      )}
    >
      <ChannelList server={ "@me" } />
      <Chat id={ props.channel.clone() } />
    </div>
  }
}
