
use dioxus::prelude::*;
use crate::{backend::server_functions::adresse_fns::save_adresse, Adresse};

#[component]
pub fn Add() -> Element {
    let mut vorname = use_signal(|| String::new());
    let mut nachname = use_signal(|| String::new());
    let mut strasse = use_signal(|| String::new());
    let mut strassen_nr = use_signal(|| String::new());
    let mut list_signal = use_signal(|| Vec::<Adresse>::new()); 

    let nav = use_navigator();
    
    rsx! {
        div { class:"add_form",
                label { "Name" }
                br {  }
                input {  
                r#type: "text",
                value: vorname,
                oninput: move |e| vorname.set(e.value()),
                }
                br {  }
                label { "Nachname" }
                br {  }
                input {  
                r#type: "text",
                value: nachname,
                
                oninput: move |e| nachname.set(e.value()),
                }
                br {  }
                label { "Strasse" } 
                br {  }
                input {  
                r#type: "text",
                value: strasse,
             
                oninput: move |e| strasse.set(e.value()),
                } 
                br {  }
                label { "Strassen-Nr." }
                br {  }
                input {  
                r#type: "number",
                value: strassen_nr,
                oninput: move |e| strassen_nr.set(e.value()),
                }  
                br {  }

                button {  class:"btn",
                    onclick: move |_| async move {
                        let save_vorname = vorname.read().clone();
                        let save_nachname = nachname.read().clone();
                        let save_strasse = strasse.read().clone();
                        let save_strassen_nr = strassen_nr.read().parse::<i32>().unwrap_or(0);
                        
                        // HIER: Das Ergebnis loggen
                        match save_adresse(save_vorname.clone(), save_nachname.clone(), save_strasse.clone(), save_strassen_nr.clone()).await {
                            Ok(id) => {
                                println!("Erfolg! ID: {}", id); // Ausgabe in der Browser-Konsole
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
                                // WICHTIG: Fehler ausgeben!
                                println!("FEHLER beim Speichern: {:?}", e); 
                                // Optional: Alert im Browser anzeigen (via web-sys oder gloo)
                            }
                        }
                        vorname.set(String::new());
                        nachname.set(String::new());
                        strasse.set(String::new());
                        strassen_nr.set(String::new());

                        
                    },
                    disabled: if vorname.read().trim().is_empty() || nachname.read().trim().is_empty() { true } else { false },
                    "Speichern"
                }
        }
        // ... (Rest des Codes für die Liste)
        Link { 
            class:"btn",
            to: "/", "Abbrechen" 
        }
    }
}


