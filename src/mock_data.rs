use crate::models::Order;

pub fn create_mock_orders() -> Vec<Order> {
  let mut mock_orders: Vec<Order> = Vec::new();
  mock_orders.push(Order {
    id: 0,
    content: "dboll".to_string(),
    is_done: false,
  });
  mock_orders.push(Order {
    id: 1,
    content: "macka".to_string(),
    is_done: false,
  });
  mock_orders.push(Order {
    id: 2,
    content: "pölse".to_string(),
    is_done: false,
  });
  mock_orders.push(Order {
    id: 3,
    content: "matlåda".to_string(),
    is_done: false,
  });
  return mock_orders;
}
