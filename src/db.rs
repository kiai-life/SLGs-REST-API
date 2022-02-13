use actix_web::Result;
use chrono::{DateTime, FixedOffset, Utc};
use data_encoding::HEXUPPER;
use postgres::{Client, NoTls};
use ring::digest::{digest, SHA256};
use serde_json::Value;

use crate::error::ApiError;

const POSTGRESQL_HOST_KEY: &str = "SLGs_POSTGRESQL_HOST";
const DEFAULT_POSTGRESQL_HOST: &str = "localhost";

const POSTGRESQL_USER_KEY: &str = "SLGs_POSTGRESQL_USER";
const DEFAULT_POSTGRESQL_USER: &str = "postgres";

const POSTGRESQL_PASSWORD_KEY: &str = "SLGs_POSTGRESQL_PASSWORD";
const DEFAULT_POSTGRESQL_PASSWORD: &str = "postgres";

const POSTGRESQL_DB_NAME_KEY: &str = "SLGs_POSTGRESQL_DB_NAME";
const DEFAULT_POSTGRESQL_DB_NAME: &str = "slgs_db";

fn make_password_hash(password: &str) -> String {
  let hash_sha256 = digest(&SHA256, password.as_bytes());
  let hash_sha256_str = HEXUPPER.encode(hash_sha256.as_ref());
  hash_sha256_str
}

fn make_db_clinet() -> Result<Client, ApiError> {
  let postgresql_host =
    std::env::var(POSTGRESQL_HOST_KEY).unwrap_or_else(|_| DEFAULT_POSTGRESQL_HOST.to_string());
  let postgresql_user =
    std::env::var(POSTGRESQL_USER_KEY).unwrap_or_else(|_| DEFAULT_POSTGRESQL_USER.to_string());
  let postgresql_password = std::env::var(POSTGRESQL_PASSWORD_KEY)
    .unwrap_or_else(|_| DEFAULT_POSTGRESQL_PASSWORD.to_string());
  let postgresql_db_name = std::env::var(POSTGRESQL_DB_NAME_KEY)
    .unwrap_or_else(|_| DEFAULT_POSTGRESQL_DB_NAME.to_string());
  // https://docs.rs/postgres/latest/postgres/config/struct.Config.html
  // を見よ
  Client::connect(
    &format!("host={postgresql_host} user={postgresql_user} password={postgresql_password} dbname={postgresql_db_name}"),
    NoTls,
  ).map_err(|e| ApiError::DataBase(None, e))
}

pub async fn register_user_data(id: &str, email: &str, password: &str) -> Result<(), ApiError> {
  let mut client = make_db_clinet()?;
  client
    .batch_execute(
      "
    CREATE TABLE IF NOT EXISTS login_data (
      id TEXT PRIMARY KEY,
      email TEXT NOT NULL,
      password_hash TEXT NOT NULL
    )
  ",
    )
    .map_err(|e| ApiError::DataBase(None, e))?;
  let password_hash = make_password_hash(password);
  client
    .execute(
      "INSERT INTO login_data (id, email, password_hash) VALUES ($1, $2, $3)",
      &[&id, &email, &password_hash],
    )
    .map_err(|e| ApiError::DataBase(None, e))?;
  client
    .batch_execute(&format!(
      "CREATE TABLE {id}_user_data (
      name TEXT PRIMARY KEY,
      data JSON NOT NULL,
      timestamp TIMESTAMPTZ NOT NULL,
    )"
    ))
    .map_err(|e| ApiError::DataBase(None, e))?;
  Ok(())
}

pub async fn check_user_password(id: &str, password: &str) -> Result<bool, ApiError> {
  let mut client = make_db_clinet()?;
  if let Some(row) = (client
    .query("SELECT * FROM login_data WHERE id = $1", &[&id])
    .map_err(|e| ApiError::DataBase(None, e))?)
  .into_iter()
  .next()
  {
    let password_hash: &str = row.get(2);
    let input_password_hash = &make_password_hash(password);
    Ok(password_hash == input_password_hash)
  } else {
    Err(ApiError::NotFoundUserId)
  }
}

pub async fn get_data(
  id: &str,
  password: &str,
  name: &str,
) -> Result<(Value, DateTime<FixedOffset>), ApiError> {
  let mut client = make_db_clinet()?;
  if check_user_password(id, password).await? {
    if let Some(row) = (client
      .query(
        &format!("SELECT * FROM {id}_user_data WHERE name=$1"),
        &[&name],
      )
      .map_err(|e| ApiError::DataBase(None, e))?)
    .into_iter()
    .next()
    {
      let data: Value = row.get(1);
      let timestamp: DateTime<FixedOffset> = row.get(2);
      Ok((data, timestamp))
    } else {
      Err(ApiError::NotFoundUserId)
    }
  } else {
    Err(ApiError::InvalidPassword)
  }
}

pub async fn insert_data(
  id: &str,
  password: &str,
  name: &str,
  data: &Value,
) -> Result<(), ApiError> {
  let mut client = make_db_clinet()?;
  if check_user_password(id, password).await? {
    let now = Utc::now().with_timezone(&FixedOffset::east(9 * 3600));
    client
      .execute(
        &format!("INSERT INTO {id}_user_data (name, data, timestamp) VALUES ($1, $2, $3)"),
        &[&name, &data, &now],
      )
      .map_err(|e| ApiError::DataBase(None, e))?;
    Ok(())
  } else {
    Err(ApiError::InvalidPassword)
  }
}

pub async fn update_data(
  id: &str,
  password: &str,
  name: &str,
  data: &Value,
) -> Result<(), ApiError> {
  let mut client = make_db_clinet()?;
  if check_user_password(id, password).await? {
    let now = Utc::now().with_timezone(&FixedOffset::east(9 * 3600));
    client
      .execute(
        &format!("UPDATE {id}_user_data SET data=$1 timestamp=$2 WHERE name=$3"),
        &[&data, &now, &name],
      )
      .map_err(|e| ApiError::DataBase(None, e))?;
    Ok(())
  } else {
    Err(ApiError::InvalidPassword)
  }
}

pub async fn delete_data(id: &str, password: &str, name: &str) -> Result<(), ApiError> {
  let mut client = make_db_clinet()?;
  if check_user_password(id, password).await? {
    client
      .execute(
        &format!("DELETE FROM {id}_user_data WHERE name=$1"),
        &[&name],
      )
      .map_err(|e| ApiError::DataBase(None, e))?;
    Ok(())
  } else {
    Err(ApiError::InvalidPassword)
  }
}
