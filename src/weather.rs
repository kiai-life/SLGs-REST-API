use actix_web::{get, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json;

use crate::error::{ApiError, GetWeatherError};

mod city_id;

/// ref: https://github.com/kiai-life/SLGs-REST-API/issues/1
#[derive(Deserialize, Clone)]
pub struct GetWeatherQuery {
  pub date: String,
  pub city: String,
}

/// ref: https://github.com/kiai-life/SLGs-REST-API/issues/1
#[derive(Serialize, Clone, Debug)]
pub struct ReturnWeatherData {
  pub ok: bool,
  pub date: String,
  pub city: String,
  pub weather: String,
  pub chance_of_rain: Option<serde_json::Value>,
  pub copyright: Option<serde_json::Value>,
}

/// ref: https://weather.tsukumijima.net/
#[get("/weather")]
pub async fn get_weather(query: web::Query<GetWeatherQuery>) -> Result<HttpResponse, ApiError> {
  let city_id_opt = city_id::find_city_id(&query.city);
  match city_id_opt {
    Some(city_id) => {
      let weather_json = crate::url_get_json(&format!(
        "https://weather.tsukumijima.net/api/forecast?city={}",
        city_id
      ))
      .await?;
      match weather_json
        .get("forecasts")
        .map(|v| {
          v.as_array()
            .map(|l| {
              l.iter()
                .find(|v| v.get("date").map(|v| v.as_str() == Some(&query.date)) == Some(true))
            })
            .flatten()
        })
        .flatten()
      {
        Some(v) => {
          let data = ReturnWeatherData {
            ok: true,
            date: query.clone().date,
            city: weather_json
              .get("location")
              .map(|v| v.get("city").map(|v| v.as_str()))
              .flatten()
              .flatten()
              .unwrap()
              .to_string(),
            weather: v
              .get("telop")
              .map(|v| v.as_str())
              .flatten()
              .unwrap()
              .to_string(),
            chance_of_rain: v.get("chanceOfRain").cloned(),
            copyright: weather_json.get("copyright").cloned(),
          };
          let body = serde_json::to_string(&data).map_err(|e| ApiError::SerdeJsonError(e))?;
          Ok(
            HttpResponse::Ok()
              .content_type("application/json")
              .body(body),
          )
        }
        _ => {
          // 日付が存在しない
          Err(ApiError::APIGetWeather(GetWeatherError::Date))
        }
      }
    }
    None => {
      // 地名があっていない
      Err(ApiError::APIGetWeather(GetWeatherError::City))
    }
  }
}
