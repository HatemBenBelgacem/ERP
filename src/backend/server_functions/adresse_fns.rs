use dioxus::prelude::*;
use crate::backend::models::adresse::Adresse;


#[cfg(feature = "server")]
use super::super::{db::get_db};


#[server]
pub async fn save_adresse(vorname: String, nachname: String, strasse: String, strassen_nr:i32) -> Result<i64, ServerFnError> {
  let db = get_db().await;

  let result = sqlx::query("INSERT INTO adresse (vorname, nachname, strasse, strassen_nr) VALUES(?, ?, ?, ?)")
        .bind(&vorname)
        .bind(&nachname)
        .bind(&strasse)
        .bind(&strassen_nr)
        .execute(db)
        .await
        .unwrap();

  Ok(result.last_insert_rowid())
}

#[server]
pub async fn delete_adresse(id:i64) -> Result<(), ServerFnError> {
    let db = get_db().await;

    sqlx::query("DELETE FROM adresse WHERE id = ?")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(())
}

#[server]
pub async fn detail_adresse(id: i64) -> Result<Adresse, ServerFnError> {
    let db = get_db().await;

    let adresse = sqlx::query("SELECT * FROM adresse WHERE id = ?")
        .bind(id)
        .fetch_on(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    ok(adresse)
}

#[server]
pub async fn adress_liste() -> Result<Vec<Adresse>, ServerFnError> {
    let db = get_db().await;


    // Achten Sie darauf: SELECT name, nicht vorname (so heißt es in Ihrer DB)
    let rows = sqlx::query_as::<_, Adresse>("SELECT id, vorname, nachname, strasse, strassen_nr FROM adresse")
        .fetch_all(db)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // 2. Wir wandeln (mappen) die DB-Daten in das Frontend-Format um
    let adressen: Vec<Adresse> = rows.into_iter().map(|sql_row| Adresse {
        id: sql_row.id,
        vorname: sql_row.vorname, // Hier mappen wir: DB 'name' -> Frontend 'vorname'
        nachname: sql_row.nachname,
        strasse: sql_row.strasse,
        strassen_nr: sql_row.strassen_nr
    }).collect();

    Ok(adressen)
}


