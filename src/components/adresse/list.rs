use dioxus::{html::form, prelude::*};


use crate::backend::{server_functions::adresse_fns::adress_liste};
use crate::components::adresse::delete_adresse::Delete;




#[component]
pub fn AdressListe() -> Element {
    let adressen_resource = use_resource(move || async move {
        adress_liste().await
    });

    rsx! {
        div {
            class:"functions",
            Link { 
                class:"btn",
                to: "/adressen/add", "Neu",
            }
            Link { 
                class: "btn",
                to: "/", "Zurück" 
            } 
        }
        div {
            match &*adressen_resource.read_unchecked() {
                // 1. Erfolgreich geladen (Some -> Ok)
                Some(Ok(adressen)) => rsx! {
                    if adressen.is_empty() {
                        div { "Keine Adressen gefunden." }
                    } else {
                        table {

                            thead {
                                tr {
                                    th { "ID" }
                                    th { "Vorname" }
                                    th { "Nachname" }
                                    th { "Strasse" }
                                    th { "Strasse-Nr." }
                                    th { "Aktion" }
                                }
                            }
                            tbody {
                                for adresse in adressen {
                                    tr { key: "{adresse.id}",
                                        td { "{adresse.id}" }
                                        td { "{adresse.vorname}" }
                                        td { "{adresse.nachname}" }
                                        td { "{adresse.strasse}" }
                                        td { "{adresse.strassen_nr}" }
                                        td {  Delete{adresse_resource: adressen_resource, id: adresse.id}}
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
       
    }
}