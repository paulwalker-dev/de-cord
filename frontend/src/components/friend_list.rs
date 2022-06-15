use yew::prelude::*;

#[function_component(FriendList)]
pub fn friend_list() -> Html {
  html! {
    <main class={classes!(
      "flex-1",
      "flex",
      "p-3",
      // Remove this line later
      "items-center"
    )}>
      // Remove this ->
      <p
        class={classes!(
          "flex-1",
          "font-bold",
          "text-xl",
          "text-center"
        )}
      >
        { "This app is non-functional" }
      </p>
      // <- Remove that
    </main>
  }
}
