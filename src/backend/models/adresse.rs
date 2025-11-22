#[cfg(feature = "server")]
#[derive(sqlx::FromRow, Debug)] // Debug ist hilfreich
pub struct AdresseSql {
  pub id : i64,
  pub name: String // WAR: vorname -> MUSS name heißen, wie in der DB
}