use reqwasm::http::Request;
use serde::de::DeserializeOwned;

pub fn get_host() -> &'static str {
  #[cfg(not(debug_assertions))]
  let host: &str = "http://localhost:8080";
  // let host: &str = "https://de-cord.paulwalker.dev";

  #[cfg(debug_assertions)]
  let host: &str = "http://localhost:8080";

  host
}

pub async fn get<T: DeserializeOwned>(url: &str) -> Result<T, reqwasm::Error> {
  let responce: T = Request::get(&format!("{}/api/{url}", get_host()))
    .send()
    .await?
    .json()
    .await?;

  Ok(responce)
}
