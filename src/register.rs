use actix_web::{get, post, web, Error, HttpResponse};
use serde::{Deserialize, Serialize};

/// ref: https://weather.tsukumijima.net/
#[get("/register")]
pub async fn register_user() -> Result<HttpResponse, Error> {
  Ok(
    HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(include_str!("./register/form.html")),
  )
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UserData {
  pub id: String,
  pub email: String,
  pub password: String,
}

#[post("/register/post_user")]
pub async fn post_user_data(params: web::Form<UserData>) -> HttpResponse {
  use crate::db;
  let res = db::register_user_data(&params.id, &params.email, &params.password).await;
  match res {
    Ok(_) => HttpResponse::Ok()
      .content_type("text/html; charset=utf-8")
      .body(include_str!("./register/success.html")),
    Err(_) => HttpResponse::InternalServerError().finish(),
  }
}
