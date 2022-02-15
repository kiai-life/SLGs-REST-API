use actix_web::{post, web, HttpRequest, HttpResponse, Result};
use serde::Deserialize;
use serde_json;

use crate::{db, error::ApiError};

const CAR_ID_DATA_NAME: &str = "car_id_data";

// https://github.com/kiai-life/SLGs-REST-API/issues/11
#[derive(Deserialize)]
pub struct CarInfo {
  id: String,
  car_model: String,
  fuel_type: String,
  manufacture: String,
}

/// ref: https://weather.tsukumijima.net/
#[post("/car/register")]
pub async fn register_car_id(
  req: HttpRequest,
  body: web::Json<CarInfo>,
) -> Result<HttpResponse, ApiError> {
  let headers = req.headers();
  let user_id = crate::get_header_str(headers, "x-slgs-user-id").await?;
  let user_password = crate::get_header_str(headers, "x-slgs-user-password").await?;
  let json = serde_json::json!({
    "id": body.id,
    "car_model" : body.car_model,
    "fuel_type" : body.fuel_type,
    "manufacture" : body.manufacture,
  });
  db::insert_data(&user_id, &user_password, CAR_ID_DATA_NAME, &json).await?;
  Err(ApiError::NotFoundUserId)
}
