use dioxus::prelude::*;

use crate::{backend::server_functions::auftrag_fns::auftrag_liste};


#[component]
pub fn AuftragListe() -> Element {
    let auftrag_resource = use_resource(move || async move {
        auftrag_liste().await
    });

    rsx! {
        div {
            class:"functions",
            Link { 
                class:"btn",
                to: "/auftrag/add", "Neu",
            }
            Link { 
                class: "btn",
                to: "/", "Zurück" 
            } 
        }

        div {
            match &*auftrag_resource.read_unchecked() {
                Some(Ok(auftrag)) => rsx! {
                    if auftrag.is_empty() {
                        div{"Keine Aufträge gefunden"}
                    } else {
                        table {
                            thead{
                                tr{
                                    th {"ID"}
                                    th {"Bezeichnung"}
                                    th{"Kunde"}
                                }
                            }
                            tbody{
                                for a in auftrag {
                                    tr{ key: "{a.id}",
                                    td {"{a.id}"}
                                    td {"{a.bezeichnung}"}
                                    td {"{a.adresse_id}"}
                                    }
                                }
                            }
                        }
                    }
                },

                Some(Err(e)) => rsx! {
                    div {  
                        style: "color: red", 
                        "Fehler beim Laden von Aufträgen: {e}"
                    }
                }, 

                None => rsx! {
                    div {"Lade Daten..."}
                }
            }
        }
    }
}