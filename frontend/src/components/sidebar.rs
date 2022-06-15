use yew::prelude::*;

#[function_component(Sidebar)]
pub fn sidebar() -> Html {
  html! {
    <nav class={classes!(
      "flex",
      "flex-col",
      "h-screen",
      "p-2",
      "bg-slate-700"
    )}>
      <div class={classes!(
        "w-12",
        "h-12",
        "bg-slate-600",
        "rounded-xl"
      )} />
    </nav>
  }
}
