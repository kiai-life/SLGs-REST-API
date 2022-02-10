use actix_web::{
  client::{Client, Connector},
  Error,
};
use openssl::ssl::{SslConnector, SslMethod};
use serde_json;

pub mod weather;

async fn url_get_json(url: &str) -> Result<serde_json::Value, Error> {
  let builder = SslConnector::builder(SslMethod::tls()).unwrap();
  let client = Client::builder()
    .connector(Connector::new().ssl(builder.build()).finish())
    .finish();
  let mut res = client.get(url).send().await?;
  let res_json = res.json::<serde_json::Value>().await?;
  Ok(res_json)
}
