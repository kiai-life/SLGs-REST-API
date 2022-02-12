use data_encoding::HEXUPPER;
use postgres::{Client, Error, NoTls};
use ring::digest::{digest, SHA256};

pub async fn register_user_data(id: &str, email: &str, password: &str) -> Result<(), Error> {
  // https://docs.rs/postgres/latest/postgres/config/struct.Config.html
  // を見よ
  let mut client = Client::connect("host=localhost user=postgres password=postgres", NoTls)?;
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
