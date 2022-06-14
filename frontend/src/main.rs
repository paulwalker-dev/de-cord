use yew::prelude::*;
use yew_router::prelude::*;

pub mod components;
pub mod lib;
mod pages;

use components::sidebar::Sidebar;
use pages::index::Index;

#[derive(Clone, Routable, PartialEq)]
enum Route {
  #[at("/")]
  Home,
}

fn switch(routes: &Route) -> Html {
  match routes {
    Route::Home => html! { <Index /> },
  }
}

#[function_component(App)]
fn app() -> Html {
  html! {
    <div
      class={classes!(
        "flex",
        "flex-row",
        "bg-slate-900",
        "text-blue-100"
      )}
    >
      <Sidebar />
      <BrowserRouter>
        <Switch<Route> render={Switch::render(switch)} />
      </BrowserRouter>
    </div>
  }
}

fn main() {
  yew::start_app::<App>();
}
