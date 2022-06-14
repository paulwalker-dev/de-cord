use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ChatProps {
  pub id: String,
}

#[function_component(Chat)]
pub fn chat(props: &ChatProps) -> Html {
  html! {
    <main
      class={classes!(
        "flex-1"
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
          { &props.id }
        </p>
      </div>
    </main>
  }
}
