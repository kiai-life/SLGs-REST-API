use actix_web::{
  client::{Client, Connector},
  Result,
};
use openssl::ssl::{SslConnector, SslMethod};
use serde_json::Value;

pub mod api;
pub mod db;
pub mod error;
pub mod register;

use error::ApiError;

async fn url_get_json(url: &str) -> Result<Value, ApiError> {
  let builder = SslConnector::builder(SslMethod::tls()).map_err(ApiError::SSLBuilder)?;
  let client = Client::builder()
    .connector(Connector::new().ssl(builder.build()).finish())
    .finish();
  let mut res = client
    .get(url)
    .send()
    .await
    .map_err(ApiError::SendRequestError)?;
  let res_json = res
    .json::<Value>()
    .await
    .map_err(ApiError::JsonPayloadError)?;
  Ok(res_json)
}
