use dioxus::prelude::*;
use crate::backend::models::auftrag::Auftrag;


#[cfg(feature = "server")]
use super::super::{db::get_db};

#[server]
pub async fn save_auftrag(bezeichnung: String, adresse_id: i64) -> Result<i64, ServerFnError> {
  let db = get_db().await;

  let result = sqlx::query("INSERT INTO auftrag (bezeichnung, adresse_id) VALUES(?, ?)")
        .bind(&bezeichnung)
        .bind(&adresse_id)
        .execute(db)
        .await
        .unwrap();

  Ok(result.last_insert_rowid())
}

#[server]
pub async fn auftrag_liste() -> Result<Vec<Auftrag>, ServerFnError> {
    let db = get_db().await;


    // Achten Sie darauf: SELECT name, nicht vorname (so heißt es in Ihrer DB)
    let rows = sqlx::query_as::<_, Auftrag>("SELECT id, bezeichnung, adresse_id FROM auftrag")
        .fetch_all(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // 2. Wir wandeln (mappen) die DB-Daten in das Frontend-Format um
    let auftrag: Vec<Auftrag> = rows.into_iter().map(|sql_row| Auftrag {
        id: sql_row.id,
        bezeichnung: sql_row.bezeichnung, 
        adresse_id: sql_row.adresse_id,
    }).collect();

    Ok(auftrag)
}


