use actix_web::{
  error,
  http::{header::ContentType, StatusCode},
  HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum UserError {
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
