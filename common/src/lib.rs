use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct User {
  pub id: String,
  pub username: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Message {
  pub user: User,
  pub contents: String,
}
