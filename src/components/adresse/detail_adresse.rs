use dioxus::prelude::*;
use crate::backend::server_functions::adresse_fns::detail_adresse;
use crate::Route;

#[component]
pub fn AdresseDetail(id: i64) -> Element {
    let adresse_resource = use_resource(move || async move {
        detail_adresse(id).await
    });

    rsx! {
        // Ein zentrierter Kasten auf der Seite, um die Ansicht kompakt zu halten
        div { class: "container mt-5",

            // Die Hauptkarte (Card) für das Design
            div { class: "card shadow-sm",

                match &*adresse_resource.read_unchecked() {
                    // 1. ERFOLG: Die Adresse wurde erfolgreich geladen
                    Some(Ok(adresse)) => rsx! {
                        // Ein blauer Kopfbereich für die Karte
                        div { class: "card-header bg-dark text-white",
                            h4 { class: "mb-0", "Adressdetails" }
                        }

                        // Der Hauptteil mit den strukturierten Daten
                        div { class: "card-body",

                            // Vorname
                            div { class: "mb-3 pb-2 border-bottom",
                                small { class: "text-muted d-block", "Vorname" }
                                span { class: "fs-5 fw-medium", "{adresse.vorname}" }
                            }

                            // Nachname
                            div { class: "mb-3 pb-2 border-bottom",
                                small { class: "text-muted d-block", "Nachname" }
                                span { class: "fs-5 fw-medium", "{adresse.nachname}" }
                            }

                            // Straße und Hausnummer
                            div { class: "mb-2",
                                small { class: "text-muted d-block", "Strasse & Hausnummer" }
                                span { class: "fs-5 fw-medium", "{adresse.strasse} {adresse.strassen_nr}" }
                            }
                        }

                        // Der Fußbereich für den Zurück-Button
                        div { class: "card-footer bg-light text-end",
                            Link { class: "btn btn-secondary", to: Route::AdressListe {}, "Zurück zur Liste" }
                        }
                    },

                    // 2. FEHLER: Beim Laden ist ein Fehler aufgetreten
                    Some(Err(e)) => rsx! {
                        div { class: "card-body text-center p-4",
                            // Eine rote Bootstrap-Warnbox (Alert)
                            div { class: "alert alert-danger mb-3", role: "alert",
                                h5 { class: "alert-heading fw-bold", "Fehler beim Laden" }
                                p { class: "mb-0 small", "{e}" }
                            }
                            Link { class: "btn btn-outline-secondary btn-sm", to: Route::AdressListe {}, "Zurück zur Liste" }
                        }
                    },

                    // 3. LADEZUSTAND: Die Daten werden noch vom Server abgerufen
                    None => rsx! {
                        div { class: "card-body text-center py-5",
                            // Ein animierter Lade-Kreis (Spinner) von Bootstrap
                            div { class: "spinner-border text-primary", role: "status",
                                span { class: "visually-hidden", "Lade..." }
                            }
                            p { class: "text-muted mt-3 mb-0", "Lade Details..." }
                        }
                    },
                }
            }
        }
    }
}