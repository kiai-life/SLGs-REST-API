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
