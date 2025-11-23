use dioxus::prelude::*;


#[cfg(feature = "server")]
use super::super::{db::get_db, models::adresse::AdresseSql};
use crate::Adresse;


#[server]
pub async fn save_adresse(vorname: String, nachname: String) -> Result<i64, ServerFnError> {
  let db = get_db().await;

  let result = sqlx::query("INSERT INTO adresse (vorname, nachname) VALUES(?, ?)").bind(&vorname).bind(&nachname).execute(db).await.unwrap();

  Ok(result.last_insert_rowid())
}

#[server]
pub async fn adress_liste() -> Result<Vec<Adresse>, ServerFnError> {
    let db = get_db().await;

    // 1. Wir laden die Daten in das 'AdresseSql' Modell (das hat FromRow!)
    // Achten Sie darauf: SELECT name, nicht vorname (so heißt es in Ihrer DB)
    let rows = sqlx::query_as::<_, AdresseSql>("SELECT id, vorname, nachname FROM adresse")
        .fetch_all(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // 2. Wir wandeln (mappen) die DB-Daten in das Frontend-Format um
    let adressen: Vec<Adresse> = rows.into_iter().map(|sql_row| Adresse {
        id: sql_row.id,
        vorname: sql_row.vorname, // Hier mappen wir: DB 'name' -> Frontend 'vorname'
        nachname: sql_row.nachname,
    }).collect();

    Ok(adressen)
}


