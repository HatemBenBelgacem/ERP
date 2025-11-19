use dioxus::prelude::*;
use crate::Adresse;

#[cfg(feature = "server")]
use super::{db::get_db, model::AdresseSql};


#[server]
pub async fn add_new_adresse(content: String, completed: bool) -> Result<i32, ServerFnError> {
  let db = get_db().await;

  let result = sqlx::query("INSERT INTO adresse (vorname, nachname) VALUES(?1, ?2)").bind(&content).bind(&completed).execute(db).await.unwrap();

  Ok(result.last_insert_rowid())
}

#[server]
pub async fn get_adresse_liste () -> Result<Vec<Adresse>, ServerFnError> {
  let db = get_db().await;
  
  let rows : Vec<ToDoSql> = sqlx::query_as("SELECT * FROM adresse").fetch_all(db).await.unwrap();

  let mut v = vec![];

  for row in rows {
    let todo = Adresse {
      id: row.id,
      vorname: row.vorname,
      nachname: row.nachname
    };
    v.push(todo);
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
    let todo = ToDo {
      id: rows[0].id,
      vorname: rows[0].content.clone(),
      nachname: rows[0].completed
    };
    Ok(todo)
  }
}