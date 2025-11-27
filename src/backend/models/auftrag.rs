use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// Füge das FromRow-Makro nur hinzu, wenn das "server"-Feature aktiv ist
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Auftrag {
  pub id : i64,
  pub bezeichnung: String, 
  pub adresse_id: i64,
}