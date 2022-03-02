use actix_web::{
    delete, error, get,
    http::{header::ContentType, StatusCode},
    post, put, web, App, HttpResponse, HttpServer, Responder, Result,
};
use derive_more::{Display, Error};
use serde::Serialize;
use std::sync::Mutex;

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "Validation error: {}", message)]
    ValidationError { message: String },
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::ValidationError { .. } => StatusCode::BAD_REQUEST,
        }
    }
}
#[derive(Clone, Serialize)]
struct Order {
    id: usize,
    content: String,
    is_done: bool,
}

struct AppState {
    orders: Mutex<Vec<Order>>,
}

#[put("/order/{order_id}/done")]
async fn mark_done(
    data: web::Data<AppState>,
    path: web::Path<usize>,
) -> Result<impl Responder, UserError> {
    let mut orders_mut = data.orders.lock().unwrap();
    let order_id = path.into_inner();
    let order_to_update = orders_mut.iter().find(|&x| x.id == order_id);
    match order_to_update {
        Some(order) => {
            let index = orders_mut.iter().position(|x| x.id == order_id).unwrap();
            let new_order = Order {
                id: order.id,
                content: order.clone().content,
                is_done: true,
            };
            orders_mut[index] = new_order.clone();
            return Ok(web::Json(new_order));
        }
        None => {
            return Err(UserError::ValidationError {
                message: format!("There is no order with the id: {}", order_id),
            })
        }
    }
}

#[delete("/order/{order_id}")]
async fn delete_order(
    data: web::Data<AppState>,
    path: web::Path<usize>,
) -> Result<impl Responder, UserError> {
    let mut orders_mut = data.orders.lock().unwrap();
    let orders = orders_mut.clone();
    let order_id = path.into_inner();
    let order_to_delete = orders.iter().find(|&x| x.id == order_id);
    match order_to_delete {
        Some(order) => {
            let index = orders.iter().position(|x| x.id == order_id).unwrap();
            orders_mut.remove(index);
            return Ok(web::Json(order.clone()));
        }
        None => {
            return Err(UserError::ValidationError {
                message: format!("There is no order with the id: {}", order_id),
            })
        }
    }
}

#[post("/order/{order}")]
async fn post_order(data: web::Data<AppState>, path: web::Path<String>) -> Result<impl Responder> {
    let mut orders = data.orders.lock().unwrap();
    let order = path.into_inner();
    let new_order = Order {
        id: orders.len(),
        content: order,
        is_done: false,
    };
    orders.push(new_order.clone());
    Ok(web::Json(new_order))
}

#[get("/orders")]
async fn get_orders(data: web::Data<AppState>) -> Result<impl Responder> {
    let orders = data.orders.lock().unwrap();
    Ok(web::Json(orders.clone()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let orders = web::Data::new(AppState {
        orders: Mutex::new(Vec::new()),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(orders.clone())
            .service(post_order)
            .service(get_orders)
            .service(mark_done)
            .service(delete_order)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
