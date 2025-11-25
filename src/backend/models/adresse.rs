use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// Füge das FromRow-Makro nur hinzu, wenn das "server"-Feature aktiv ist
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Adresse {
  pub id : i64,
  pub vorname: String, 
  pub nachname: String, 
  pub strasse: String,
  pub strassen_nr: i32
}