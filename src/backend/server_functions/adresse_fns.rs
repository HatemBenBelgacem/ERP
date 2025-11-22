use dioxus::prelude::*;


#[cfg(feature = "server")]
use super::super::{db::get_db, models::adresse::AdresseSql};


#[server]
pub async fn save_name(name: String) -> Result<i64, ServerFnError> {
  let db = get_db().await;

  let result = sqlx::query("INSERT INTO adresse (name) VALUES(?)").bind(&name).execute(db).await.unwrap();

  Ok(result.last_insert_rowid())
}



