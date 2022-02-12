use actix_web::{get, middleware::Logger, App, Error, HttpResponse, HttpServer, web};
use slgs_rest_api::{register, weather};

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
  std::env::set_var("RUST_LOG", "info");
  env_logger::init();
  HttpServer::new(|| {
    App::new()
      .wrap(Logger::default())
      .service(index)
      .service(web::scope("/api/v1").service(weather::get_weather))
      .service(register::post_user_data)
      .service(register::register_user)
  })
  .bind("127.0.0.1:55555")?
  .run()
  .await
}
