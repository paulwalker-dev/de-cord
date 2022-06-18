use yew::prelude::*;
use yew_router::prelude::*;

pub mod components;
pub mod lib;
mod pages;

use components::sidebar::Sidebar;
use pages::home::Home;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/channels/:server/:id")]
  Channel { server: String, id: String },
}

fn switch(routes: &Route) -> Html {
  match routes {
    Route::Home => html! {
      <Home server={ "@me".to_string() } channel={ "@friends".to_string() } />
    },
    Route::Channel { server, id } => html! {
      <Home server={ server.clone() } channel={ id.clone() } />
    },
  }
}

#[function_component(App)]
fn app() -> Html {
  html! {
    <div class={classes!(
      "flex",
      "flex-row",
      "w-screen",
      "bg-slate-900",
      "text-blue-100"
    )}>
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
