use reqwasm::http::Request;
use serde::{de::DeserializeOwned, Serialize};
use yew::prelude::*;

pub fn get_host() -> &'static str {
  #[cfg(not(debug_assertions))]
  let host: &str = "http://localhost:8080";

  #[cfg(debug_assertions)]
  let host: &str = "http://localhost:8080";

  host
}

async fn get<T: DeserializeOwned>(url: &str) -> Result<T, reqwasm::Error> {
  let responce: T = Request::get(&format!("{}/api/{url}", get_host()))
    .send()
    .await?
    .json()
    .await?;

  Ok(responce)
}

async fn post<T: Serialize>(url: &str, contents: &T) -> Result<(), reqwasm::Error> {
  Request::post(&format!("{}/api/{url}", get_host()))
    .body(serde_json::to_string(contents)?)
    .send()
    .await?
    .json()
    .await?;

  Ok(())
}

pub fn post_api<T>(url: &str, contents: &T) -> Result<(), reqwasm::Error>
where
  T: Serialize + Clone + 'static,
{
  let url = url.to_string();
  let contents = contents.clone();
  wasm_bindgen_futures::spawn_local(async move {
    post(&url, &contents).await.unwrap();
  });

  Ok(())
}

pub fn use_api<T, Dependents>(path: String, deps: Dependents) -> Option<T>
where
  T: DeserializeOwned + Clone + 'static,
  Dependents: PartialEq + 'static,
{
  let result: UseStateHandle<Option<T>> = use_state(|| None);
  {
    let result = result.clone();
    use_effect_with_deps(
      move |_| {
        wasm_bindgen_futures::spawn_local(async move {
          let responce: Result<T, reqwasm::Error> = get(&path).await;
          result.set(match responce {
            Ok(val) => Some(val),
            Err(_) => None,
          })
        });
        || ()
      },
      deps,
    );
  }

  (*result).clone()
}
