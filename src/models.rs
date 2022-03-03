use serde::Serialize;
use std::sync::Mutex;

#[derive(Clone, Serialize)]
pub struct Order {
  pub id: usize,
  pub content: String,
  pub is_done: bool,
}

pub struct AppState {
  pub orders: Mutex<Vec<Order>>,
}
