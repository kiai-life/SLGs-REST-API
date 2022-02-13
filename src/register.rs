use actix_web::{get, post, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

use crate::error::ApiError;

/// ref: https://weather.tsukumijima.net/
#[get("/register")]
pub async fn register_user() -> HttpResponse {
  HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(include_str!("./register/form.html"))
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserData {
  pub id: String,
  pub email: String,
  pub password: String,
}

#[post("/register/post_user")]
pub async fn post_user_data(params: web::Form<UserData>) -> Result<HttpResponse, ApiError> {
  use crate::db;
  db::register_user_data(&params.id, &params.email, &params.password).await?;
  Ok(
    HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(include_str!("./register/success.html")),
  )
}
