use dioxus::prelude::*;

use crate::{backend::server_functions::adresse_fns::adress_liste};


#[component]
pub fn AdressListe() -> Element {
    
    rsx! {
        div {
            h2 { "Adress Liste" }
            button {
                onclick: move |_| async move {
                    match adress_liste().await {
                        Ok(adressen) => {
                            for adresse in adressen {
                                println!("ID: {}, Vorname: {}, Nachname: {}", adresse.id, adresse.vorname, adresse.nachname);
                            }
                        }
                        Err(e) => {
                            println!("FEHLER beim Abrufen der Adressliste: {:?}", e);
                        }
                    }
                },
                "Lade Adressliste"
            
            }
        }
    }
}