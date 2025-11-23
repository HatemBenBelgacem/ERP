use dioxus::prelude::*;

use crate::{backend::server_functions::adresse_fns::adress_liste};



#[component]
pub fn AdressListe() -> Element {
    let adressen_resource = use_resource(move || async move {
        adress_liste().await
    });

    rsx! {
        div {
            h2 { "Adress Liste" }

            match &*adressen_resource.read_unchecked() {
                // 1. Erfolgreich geladen (Some -> Ok)
                Some(Ok(adressen)) => rsx! {
                    if adressen.is_empty() {
                        div { "Keine Adressen gefunden." }
                    } else {
                        table {
                            style: "width: 100%; border-collapse: collapse; margin-top: 20px;",
                            thead {
                                tr {
                                    th { style: "text-align: left; border-bottom: 1px solid #ddd;", "ID" }
                                    th { style: "text-align: left; border-bottom: 1px solid #ddd;", "Vorname" }
                                    th { style: "text-align: left; border-bottom: 1px solid #ddd;", "Nachname" }
                                }
                            }
                            tbody {
                                for adresse in adressen {
                                    tr { key: "{adresse.id}",
                                        td { style: "padding: 8px; border-bottom: 1px solid #444;", "{adresse.id}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #444;", "{adresse.vorname}" }
                                        td { style: "padding: 8px; border-bottom: 1px solid #444;", "{adresse.nachname}" }
                                    }
                                }
                            }
                        }
                    }
                },
                
                // 2. Fehler beim Laden vom Server (Some -> Err)
                Some(Err(e)) => rsx! {
                    div { 
                        style: "color: red;",
                        "Fehler beim Laden der Adressen: {e}" 
                    }
                },

                // 3. Daten werden noch geladen (None)
                // Hier lag der Fehler: Es ist einfach "None", nicht "Some(None)"
                None => rsx! {
                    div { "Lade Daten..." }
                }
            }
            
            
        }
        Link { to: "/adressen/add", "Adresse hinzufügen" }
        br {  }
        Link { to: "/", "Zurück zur Startseite" }
    }
}