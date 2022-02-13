use actix_web::{
  client::{Client, Connector},
  Result,
};
use openssl::ssl::{SslConnector, SslMethod};
use serde_json::Value;

pub mod db;
pub mod error;
pub mod register;
pub mod weather;

use error::ApiError;

async fn url_get_json(url: &str) -> Result<Value, ApiError> {
  let builder = SslConnector::builder(SslMethod::tls()).map_err(|e| ApiError::SSLBuilder(e))?;
  let client = Client::builder()
    .connector(Connector::new().ssl(builder.build()).finish())
    .finish();
  let mut res = client
    .get(url)
    .send()
    .await
    .map_err(|e| ApiError::SendRequestError(e))?;
  let res_json = res
    .json::<Value>()
    .await
    .map_err(|e| ApiError::JsonPayloadError(e))?;
  Ok(res_json)
}
