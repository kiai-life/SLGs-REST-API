use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(App::new)
    .bind("127.0.0.1:55555")?
    .run()
    .await
}
