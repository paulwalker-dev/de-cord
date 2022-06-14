use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

struct AppState {
  count: Mutex<i32>,
}

async fn index(data: web::Data<AppState>) -> String {
  let mut count = data.count.lock().unwrap();
  *count += 1;

  format!("{count}")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
  let data = web::Data::new(AppState {
    count: Mutex::new(0),
  });

  HttpServer::new(move || {
    App::new()
      .app_data(data.clone())
      .route("/", web::get().to(index))
  })
  .bind(("0.0.0.0", 3000))?
  .run()
  .await
}
