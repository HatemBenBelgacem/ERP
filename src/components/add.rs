
use dioxus::prelude::*;
use crate::{backend::server_functions::adresse_fns::save_adresse, Adresse};

#[component]
pub fn Add() -> Element {
    let mut vorname = use_signal(|| String::new());
    let mut nachname = use_signal(|| String::new());
    let mut list_signal = use_signal(|| Vec::<Adresse>::new()); 
    
    rsx! {
        div {
            form {
                label { "Name" }
                input {  
                r#type: "text",
                value: vorname,
                placeholder: "Name",
                oninput: move |e| vorname.set(e.value()),
            }
            label { "Nachname" }
            input {  
                r#type: "text",
                value: nachname,
                placeholder: "Nachname",
                oninput: move |e| nachname.set(e.value()),
            }  

            }
            
                
            button {  
                onclick: move |_| async move {
                    let save_vorname = vorname.read().clone();
                    let save_nachname = nachname.read().clone();
                    
                    // HIER: Das Ergebnis loggen
                    match save_adresse(save_vorname.clone(), save_nachname.clone()).await {
                        Ok(id) => {
                            println!("Erfolg! ID: {}", id); // Ausgabe in der Browser-Konsole
                            let adresse = Adresse {
                                id,
                                vorname: save_vorname,
                                nachname: save_nachname,
                            };
                            list_signal.write().push(adresse);
                        }
                        Err(e) => {
                            // WICHTIG: Fehler ausgeben!
                            println!("FEHLER beim Speichern: {:?}", e); 
                            // Optional: Alert im Browser anzeigen (via web-sys oder gloo)
                        }
                    }
                    vorname.set(String::new());
                    nachname.set(String::new());
                },
                disabled: if vorname.read().trim().is_empty() || nachname.read().trim().is_empty() { true } else { false },
                "save"
            }
        }
        // ... (Rest des Codes für die Liste)
    }
}


