#[cfg(feature = "server")]
#[derive(sqlx::FromRow)]
pub struct AdresseSql {
  pub id : i64,
  pub vorname: String
}