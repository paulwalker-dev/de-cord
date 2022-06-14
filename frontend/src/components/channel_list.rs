use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChannelListProps {
  pub server: String,
}

#[function_component(ChannelList)]
pub fn channel_list(props: &ChannelListProps) -> Html {
  html! {
    <nav
      class={classes!(
        "flex",
        "flex-col",
        "h-screen",
        "p-2",
        "bg-slate-800"
      )}
    >
      <div
        class={classes!(
          "w-40",
          "bg-slate-700",
          "px-2",
          "py-1",
          "rounded-xl"
        )}
      >
        { &props.server }
      </div>
    </nav>
  }
}
