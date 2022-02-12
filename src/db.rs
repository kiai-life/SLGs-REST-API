use data_encoding::HEXUPPER;
use postgres::{Client, Error, NoTls};
use ring::digest::{digest, SHA256};

const POSTGRESQL_HOST_KEY: &str = "SLGs_POSTGRESQL_HOST";
const DEFAULT_POSTGRESQL_HOST: &str = "localhost";

const POSTGRESQL_USER_KEY: &str = "SLGs_POSTGRESQL_USER";
const DEFAULT_POSTGRESQL_USER: &str = "postgres";

const POSTGRESQL_PASSWORD_KEY: &str = "SLGs_POSTGRESQL_PASSWORD";
const DEFAULT_POSTGRESQL_PASSWORD: &str = "postgres";

const POSTGRESQL_DB_NAME_KEY: &str = "SLGs_POSTGRESQL_DB_NAME";
const DEFAULT_POSTGRESQL_DB_NAME: &str = "slgs_db";

pub async fn register_user_data(id: &str, email: &str, password: &str) -> Result<(), Error> {
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
  let mut client = Client::connect(
    &format!("host={postgresql_host} user={postgresql_user} password={postgresql_password} dbname={postgresql_db_name}"),
    NoTls,
  )?;
  client.batch_execute(
    "
    CREATE TABLE IF NOT EXISTS login_data (
      id TEXT PRIMARY KEY,
      email TEXT NOT NULL,
      password_hash TEXT NOT NULL
    )
  ",
  )?;
  let hash_sha256 = digest(&SHA256, password.as_bytes());
  let hash_sha256_str = HEXUPPER.encode(hash_sha256.as_ref());
  client.execute(
    "INSERT INTO login_data (id, email, password_hash) VALUES ($1, $2, $3)",
    &[&id, &email, &hash_sha256_str],
  )?;
  client.batch_execute(&format!(
    "CREATE TABLE {id}_user_data (
      event_name TEXT PRIMARY KEY,
      data JSON NOT NULL
    )"
  ))?;
  Ok(())
}
