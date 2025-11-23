use dioxus::prelude::*;


#[cfg(feature = "server")]
use super::super::{db::get_db, models::adresse::AdresseSql};
use crate::Adresse;


#[server]
pub async fn save_adresse(name: String, nachname: String) -> Result<i64, ServerFnError> {
  let db = get_db().await;

  let result = sqlx::query("INSERT INTO adresse (name, nachname) VALUES(?, ?)").bind(&name).bind(&nachname).execute(db).await.unwrap();

  Ok(result.last_insert_rowid())
}

#[server]
pub async fn adress_liste() -> Result<Vec<Adresse>, ServerFnError> {

  let db = get_db().await;

  let rows = sqlx::query_as::<_, AdresseSql>("SELECT id, vorname, nachname FROM adresse")
      .fetch_all(db)
      .await
      .unwrap();

  Ok(rows)
}


