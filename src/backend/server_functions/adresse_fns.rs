use dioxus::prelude::*;
use crate::Adresse;

#[cfg(feature = "server")]
use super::super::{db::get_db, models::adresse::AdresseSql};


#[server]
pub async fn add_new_adresse(vorname: String, nachname: String) -> Result<i64, ServerFnError> {
  let db = get_db().await;

  let result = sqlx::query("INSERT INTO adresse (vorname) VALUES(?)").bind(&vorname).bind(&nachname).execute(db).await.unwrap();

  Ok(result.last_insert_rowid())
}

#[server]
pub async fn get_adresse_liste () -> Result<Vec<Adresse>, ServerFnError> {
  let db = get_db().await;
  
  let rows : Vec<AdresseSql> = sqlx::query_as("SELECT * FROM adresse").fetch_all(db).await.unwrap();

  let mut v = vec![];

  for row in rows {
    let adresse = Adresse {
      id: row.id,
      vorname: row.vorname,

    };
    v.push(adresse);
  }

  Ok(v)
}

#[server]
pub async fn get_single_adresse(id: i64) -> Result<Adresse, ServerFnError> {
  let db = get_db().await;

  let rows : Vec<AdresseSql> = sqlx::query_as("SELECT * FROM adresse WHERE id = ?1").bind(&id).fetch_all(db).await.unwrap();

  if rows.len() == 0 {
    let msg = format!("Todo id : {} Not Found.", id);
    Err(ServerFnError::new(msg))
  } else {
    let adresse = Adresse {
      id: rows[0].id,
      vorname: rows[0].vorname.clone(),

    };
    Ok(adresse)
  }
}