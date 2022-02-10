use actix_web::{get, App, Error, HttpResponse, HttpServer};

use slgs_rest_api::weather;

#[get("/ping")]
async fn index() -> Result<HttpResponse, Error> {
  Ok(
    HttpResponse::ExpectationFailed()
      .content_type("application/json")
      .body(r##"{"msg": "pong"}"##),
  )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| App::new().service(index).service(weather::get_weather))
    .bind("127.0.0.1:55555")?
    .run()
    .await
}
