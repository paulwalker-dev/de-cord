use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct RootData {
  pub users: Vec<User>,
  pub servers: Vec<Server>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
  pub id: i32,
  pub profile: UserProfile,
  pub dms: ChannelList,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Server {
  pub id: i32,
  pub owner_id: i32,
  pub members: Vec<UserProfile>,
  pub channels: ChannelList,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct ChannelList {
  pub server: String,
  pub channels: Vec<Channel>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Channel {
  pub id: i32,
  pub user_ids: Vec<i32>,
  pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct UserProfile {
  pub id: i32,
  pub username: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Message {
  pub id: i32,
  pub user_id: i32,
  pub contents: String,
}
