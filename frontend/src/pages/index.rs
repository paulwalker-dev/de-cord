use crate::lib::api::get;
use yew::prelude::*;

#[function_component(Index)]
pub fn index() -> Html {
  let count = use_state(|| 0);
  {
    let count = count.clone();
    use_effect_with_deps(
      |_| {
        wasm_bindgen_futures::spawn_local(async move {
          let responce: i32 = get("").await.unwrap();
          count.set(responce);
        });
        || ()
      },
      (),
    )
  }

  html! {
    <main>
      <h1>{ "Hello, World!" }</h1>
      <p>{ format!("Reload count: {}", *count) }</p>
    </main>
  }
}
