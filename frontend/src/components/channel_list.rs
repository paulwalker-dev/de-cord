use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChannelListProps {
  pub server: String,
}

#[function_component(ChannelList)]
pub fn channel_list(props: &ChannelListProps) -> Html {
  html! {
    <div
      class={classes!(
        "flex",
        "flex-col",
        "h-screen",
        "bg-slate-800"
      )}
    >
      <div
        class={classes!(
          "flex",
          "items-center",
          "h-12",
          "p-2",
          "border-slate-600",
          "border-b-2"
        )}
      >
        <p
          class={classes!(
            "font-bold"
          )}
        >
          { &props.server }
        </p>
      </div>
      <nav
        class={classes!(
          "p-2"
        )}
      >
        <button
          class={classes!(
            "hover:bg-slate-700",
            "w-40",
            "px-2",
            "py-1",
            "rounded-xl",
            "text-left"
          )}
        >
          { "Dummy Data" }
        </button>
      </nav>
    </div>
  }
}
