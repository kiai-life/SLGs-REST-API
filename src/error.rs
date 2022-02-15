use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone)]
pub enum GetWeatherError {
  City,
  Date,
}

#[derive(Debug, Serialize, Clone)]
pub struct ErrorBody {
  pub ok: bool,
  pub msg: String,
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApiError {
  #[error("")]
  ActixWebError(actix_web::Error),
  #[error("header failed")]
  HeaderFailed(String),
  #[error("database error")]
  DataBase(Option<ErrorBody>, #[source] postgres::Error),
  #[error("database error")]
  NotFoundUserId,
  #[error("invalid password")]
  InvalidPassword,
  #[error("SSL error")]
  SSLBuilder(#[source] openssl::error::ErrorStack),
  #[error("")]
  SendRequestError(#[source] actix_web::client::SendRequestError),
  #[error("")]
  JsonPayloadError(#[source] actix_web::client::JsonPayloadError),
  #[error("")]
  SerdeJsonError(#[source] serde_json::Error),
  #[error("")]
  APIGetWeather(GetWeatherError),
}

impl ResponseError for ApiError {
  fn status_code(&self) -> StatusCode {
    match *self {
      ApiError::ActixWebError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      ApiError::HeaderFailed(_) => StatusCode::PRECONDITION_FAILED,
      ApiError::DataBase(_, _) => StatusCode::INTERNAL_SERVER_ERROR,
      ApiError::NotFoundUserId => StatusCode::BAD_REQUEST,
      ApiError::InvalidPassword => StatusCode::BAD_REQUEST,
      ApiError::SSLBuilder(_) => StatusCode::INTERNAL_SERVER_ERROR,
      ApiError::SendRequestError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      ApiError::JsonPayloadError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      ApiError::SerdeJsonError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      ApiError::APIGetWeather(_) => StatusCode::BAD_REQUEST,
    }
  }

  fn error_response(&self) -> HttpResponse {
    match *self {
      ApiError::ActixWebError(_) => HttpResponse::build(self.status_code()).json(ErrorBody {
        ok: false,
        msg: "error".to_string(),
      }),
      ApiError::DataBase(_, _) => HttpResponse::build(self.status_code()).json(ErrorBody {
        ok: false,
        msg: "database error".to_string(),
      }),
      ApiError::HeaderFailed(_) => HttpResponse::build(self.status_code()).json(ErrorBody {
        ok: false,
        msg: "header failed".to_string(),
      }),
      ApiError::NotFoundUserId => HttpResponse::build(self.status_code()).json(ErrorBody {
        ok: false,
        msg: "not found user id".to_string(),
      }),
      ApiError::InvalidPassword => HttpResponse::build(self.status_code()).json(ErrorBody {
        ok: false,
        msg: "invalid password".to_string(),
      }),
      ApiError::SSLBuilder(_) => HttpResponse::build(self.status_code()).json(ErrorBody {
        ok: false,
        msg: "SSL error".to_string(),
      }),
      ApiError::SendRequestError(_) => HttpResponse::build(self.status_code()).json(ErrorBody {
        ok: false,
        msg: "Client error".to_string(),
      }),
      ApiError::JsonPayloadError(_) => HttpResponse::build(self.status_code()).json(ErrorBody {
        ok: false,
        msg: "json parser error".to_string(),
      }),
      ApiError::SerdeJsonError(_) => HttpResponse::build(self.status_code()).json(ErrorBody {
        ok: false,
        msg: "serde_json error".to_string(),
      }),
      ApiError::APIGetWeather(GetWeatherError::City) => HttpResponse::build(self.status_code())
        .json(ErrorBody {
          ok: false,
          msg: "invalid_city_name".to_string(),
        }),
      ApiError::APIGetWeather(GetWeatherError::Date) => HttpResponse::build(self.status_code())
        .json(ErrorBody {
          ok: false,
          msg: "date_not_found".to_string(),
        }),
    }
  }
}
