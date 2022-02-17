use actix_web::{post, web, HttpRequest, HttpResponse, Result};
use chrono::{offset::FixedOffset, DateTime, Utc};
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

// 車の登録
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
  match db::get_data(&user_id, &user_password, CAR_ID_DATA_NAME) {
    Ok((get_data, _)) => get_data
      .as_array()
      .map(|lst| {
        let mut lst = lst.clone();
        lst.push(json);
        let data = serde_json::json!(lst);
        db::update_data(&user_id, &user_password, CAR_ID_DATA_NAME, &data)
      })
      .ok_or(ApiError::InvalidData)??,
    Err(ApiError::NotFoundData) => {
      let data = serde_json::json!([json]);
      db::insert_data(&user_id, &user_password, CAR_ID_DATA_NAME, &data)?
    }
    Err(e) => return Err(e),
  };
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body(r#"{"ok": true}"#),
  )
}

const CAR_REFUEL_DATA_NAME: &str = "car_refuel_data";

// https://github.com/kiai-life/SLGs-REST-API/issues/10
#[derive(Deserialize)]
pub struct RefuelInfo {
  id: String,
  mileage: f32,
  amount_of_fuel: f64,
  date: Option<String>,
  price_per_liter: Option<f64>,
}

// 給油量と走行距離の登録
#[post("/car/refuel")]
pub async fn register_refuel_data(
  req: HttpRequest,
  body: web::Json<RefuelInfo>,
) -> Result<HttpResponse, ApiError> {
  let headers = req.headers();
  let user_id = crate::get_header_str(headers, "x-slgs-user-id").await?;
  let user_password = crate::get_header_str(headers, "x-slgs-user-password").await?;
  let date_str = match &body.date {
    Some(s) => {
      let date = DateTime::parse_from_rfc3339(s).map_err(|_| ApiError::ChronoError)?;
      date.to_rfc3339()
    }
    None => {
      let now = Utc::now().with_timezone(&FixedOffset::east(9 * 3600));
      now.to_rfc3339()
    }
  };
  let json = match body.price_per_liter {
    Some(price_per_liter) => {
      serde_json::json!({
        "id": body.id,
        "mileage": body.mileage,
        "amount_of_fuel" : body.amount_of_fuel,
        "date" : date_str,
        "price_per_liter": price_per_liter
      })
    }
    None => {
      serde_json::json!({
        "id": body.id,
        "mileage": body.mileage,
        "amount_of_fuel" : body.amount_of_fuel,
        "date" : date_str,
      })
    }
  };
  match db::get_data(&user_id, &user_password, CAR_REFUEL_DATA_NAME) {
    Ok((get_data, _)) => get_data
      .as_array()
      .map(|lst| {
        let mut lst = lst.clone();
        lst.push(json);
        let data = serde_json::json!(lst);
        db::update_data(&user_id, &user_password, CAR_REFUEL_DATA_NAME, &data)
      })
      .ok_or(ApiError::InvalidData)??,
    Err(ApiError::NotFoundData) => {
      let data = serde_json::json!([json]);
      db::insert_data(&user_id, &user_password, CAR_REFUEL_DATA_NAME, &data)?
    }
    Err(e) => return Err(e),
  };
  Ok(
    HttpResponse::Ok()
      .content_type("application/json")
      .body(r#"{"ok": true}"#),
  )
}
