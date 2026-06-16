use dioxus::prelude::*;
use crate::icons::{Icon, mdi_light};

use crate::backend::server_functions::adresse_fns::adress_liste;
use crate::components::adresse::delete_adresse::Delete;
use crate::Route;

#[component]
pub fn AdressListe() -> Element {
    let adressen_resource = use_resource(move || async move {
        adress_liste().await
    });

    rsx! {
        // --- Start des Bootstrap Menübands ---
        nav { class: "navbar navbar-expand-lg navbar-light bg-light mb-4 shadow-sm rounded",
            div { class: "container-fluid",
                // Titel des Menübands
                span { class: "navbar-brand mb-0 h1", "Adressübersicht" }

                // Menüpunkte (linksbündig)
                div { class: "collapse navbar-collapse",
                    ul { class: "navbar-nav me-auto mb-2 mb-lg-0",
                        li { class: "nav-item",
                            Link { class: "nav-link", to: "/", "Zurück zum Start" }
                        }
                    }

                    // Aktions-Button (rechtsbündig)
                    div { class: "d-flex",
                        Link {
                            class: "btn btn-success shadow-sm",
                            to: "/adressen/add",
                            "+ Neue Adresse"
                        }
                    }
                }
            }
        }
        // --- Ende des Bootstrap Menübands ---

        // Der Bereich für die Tabelle
        div { class: "container-fluid",
            match &*adressen_resource.read_unchecked() {
                // 1. Erfolgreich geladen
                Some(Ok(adressen)) => rsx! {
                    if adressen.is_empty() {
                        div { class: "alert alert-info", role: "alert",
                            "Es wurden noch keine Adressen gefunden. Bitte lege eine neue Adresse an."
                        }
                    } else {
                        div { class: "table-responsive",
                            table { class: "table table-striped table-hover align-middle",
                                thead { class: "table-light",
                                    tr {
                                        th { "ID" }
                                        th { "Vorname" }
                                        th { "Nachname" }
                                        th { "Strasse" }
                                        th { "Strasse-Nr." }
                                        th { colspan: 2, "Aktion" }
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
                                            td {
                                                Delete {
                                                    adresse_resource: adressen_resource,
                                                    id: adresse.id,
                                                }
                                            }
                                            td {
                                                Link {
                                                    class: "btn btn-sm btn-outline-primary",
                                                    to: Route::AdresseDetail {
                                                        id: adresse.id,
                                                    },
                                                    Icon { data: mdi_light::ClipboardText }
                                                    " Details"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                Some(Err(e)) => rsx! {
                    div { class: "alert alert-danger", role: "alert", "Fehler beim Laden der Adressen: {e}" }
                },
                None => rsx! {
                    div { class: "d-flex justify-content-center mt-5",
                        div { class: "spinner-border text-primary", role: "status",
                            span { class: "visually-hidden", "Lade Daten..." }
                        }
                    }
                },
            }
        }
    }
}