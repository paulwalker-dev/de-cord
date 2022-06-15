use actix_web::{get, web, App, HttpResponse, HttpServer};
use common::*;
use std::sync::Mutex;

fn get_fake_channel() -> Channel {
  Channel {
    id: 2,
    user_ids: vec![1, 2],
    messages: vec![Message {
      id: 3,
      user_id: 2,
      contents: "Hello, World!".to_string(),
    }],
  }
}

fn get_fake_root() -> RootData {
  RootData {
    users: vec![
      User {
        id: 1,
        profile: UserProfile {
          id: 1,
          username: "LinuxWizard".to_string(),
        },
        dms: ChannelList {
          server: "@me".to_string(),
          channels: vec![get_fake_channel()],
        },
      },
      User {
        id: 2,
        profile: UserProfile {
          id: 2,
          username: "TestUser".to_string(),
        },
        dms: ChannelList {
          server: "@me".to_string(),
          channels: vec![get_fake_channel()],
        },
      },
    ],
    servers: vec![],
  }
}

struct AppState {
  data: Mutex<RootData>,
}

#[get("/user/{id}")]
async fn get_user(data: web::Data<AppState>, id: web::Path<i32>) -> HttpResponse {
  match data
    .data
    .lock()
    .unwrap()
    .users
    .iter()
    .find(|user| user.id == *id)
  {
    Some(user) => HttpResponse::Ok().json(user),
    None => HttpResponse::NotFound().finish(),
  }
}

#[get("/profile/{id}")]
async fn get_profile(data: web::Data<AppState>, id: web::Path<i32>) -> HttpResponse {
  match data
    .data
    .lock()
    .unwrap()
    .users
    .iter()
    .find(|user| user.id == *id)
  {
    Some(user) => HttpResponse::Ok().json(&user.profile),
    None => HttpResponse::NotFound().finish(),
  }
}

#[get("/dms/{id}")]
async fn get_dm(data: web::Data<AppState>, id: web::Path<i32>) -> HttpResponse {
  match data.data.lock().unwrap().users[0]
    .dms
    .channels
    .iter()
    .find(|channel| channel.id == *id)
  {
    Some(user) => HttpResponse::Ok().json(user),
    None => HttpResponse::NotFound().finish(),
  }
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
  let data = web::Data::new(AppState {
    data: Mutex::new(get_fake_root()),
  });

  HttpServer::new(move || {
    App::new()
      .app_data(data.clone())
      .service(get_user)
      .service(get_profile)
      .service(get_dm)
  })
  .bind(("0.0.0.0", 3000))?
  .run()
  .await
}
