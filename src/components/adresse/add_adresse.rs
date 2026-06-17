use dioxus::prelude::*;
use crate::{backend::server_functions::adresse_fns::save_adresse, Adresse};

#[component]
pub fn AddAdresse() -> Element {
    let mut vorname = use_signal(|| String::new());
    let mut nachname = use_signal(|| String::new());
    let mut strasse = use_signal(|| String::new());
    let mut strassen_nr = use_signal(|| String::new());
    let mut list_signal = use_signal(|| Vec::<Adresse>::new()); 

    let nav = use_navigator();
    
    rsx! {
        // Ein Container zentriert den Inhalt und gibt ihm links und rechts etwas Platz
        div { class: "container mt-5",

            // Eine Karte (Card) für ein aufgeräumtes Design mit leichtem Schatten
            div { class: "card shadow-sm",

                // Der Kopfbereich der Karte
                div { class: "card-header bg-dark text-white",
                    h4 { class: "mb-0", "Neue Adresse hinzufügen" }
                }

                // Der Hauptbereich mit den Eingabefeldern
                div { class: "card-body",

                    // Vorname
                    div { class: "mb-3",
                        label { class: "form-label fw-bold", "Vorname" }
                        input {
                            class: "form-control", // Das macht das Eingabefeld "Bootstrap-schick"
                            r#type: "text",
                            value: vorname,
                            oninput: move |e| vorname.set(e.value()),
                        }
                    }

                    // Nachname
                    div { class: "mb-3",
                        label { class: "form-label fw-bold", "Nachname" }
                        input {
                            class: "form-control",
                            r#type: "text",
                            value: nachname,
                            oninput: move |e| nachname.set(e.value()),
                        }
                    }

                    // Straße und Hausnummer nebeneinander (in einer Zeile)
                    div { class: "row mb-3",
                        div { class: "col-md-9", // Nimmt 9 von 12 Spalten ein (breiter)
                            label { class: "form-label fw-bold", "Strasse" }
                            input {
                                class: "form-control",
                                r#type: "text",
                                value: strasse,
                                oninput: move |e| strasse.set(e.value()),
                            }
                        }
                        div { class: "col-md-3 mt-3 mt-md-0", // Nimmt 3 von 12 Spalten ein (schmaler)
                            label { class: "form-label fw-bold", "Nr." }
                            input {
                                class: "form-control",
                                r#type: "number",
                                value: strassen_nr,
                                oninput: move |e| strassen_nr.set(e.value()),
                            }
                        }
                    }
                }

                // Der Fußbereich der Karte für die Knöpfe
                div { class: "card-footer d-flex justify-content-between bg-light",

                    // Abbrechen-Knopf (links)
                    Link { class: "btn btn-outline-danger", to: "/", "Abbrechen" }

                    // Speichern-Knopf (rechts)
                    button {
                        class: "btn btn-success px-4", // px-4 macht den Knopf etwas breiter
                        disabled: if vorname.read().trim().is_empty() || nachname.read().trim().is_empty() { true } else { false },
                        onclick: move |_| async move {
                            let save_vorname = vorname.read().clone();
                            let save_nachname = nachname.read().clone();
                            let save_strasse = strasse.read().clone();
                            let save_strassen_nr = strassen_nr.read().parse::<i32>().unwrap_or(0);

                            // HIER: Das Ergebnis loggen
                            match save_adresse(
                                    save_vorname.clone(),
                                    save_nachname.clone(),
                                    save_strasse.clone(),
                                    save_strassen_nr.clone(),
                                )
                                .await
                            {
                                Ok(id) => {
                                    println!("Erfolg! ID: {}", id);
                                    let adresse = Adresse {
                                        id,
                                        vorname: save_vorname,
                                        nachname: save_nachname,
                                        strasse: save_strasse,
                                        strassen_nr: save_strassen_nr,
                                    };
                                    list_signal.write().push(adresse);
                                    nav.push("/adressen");
                                }
                                Err(e) => {
                                    println!("FEHLER beim Speichern: {:?}", e);
                                }
                            }
                            vorname.set(String::new());
                            nachname.set(String::new());
                            strasse.set(String::new());
                            strassen_nr.set(String::new());
                        },
                        "Speichern"
                    }
                }
            }
        }
    }
}