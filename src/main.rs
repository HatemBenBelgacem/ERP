use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use backend::server_functions::adresse_fns::save_name;

mod backend;

fn main() {
    dioxus::launch(App);
}

// src/main.rs

#[component]
fn App() -> Element {
    let mut eingabe = use_signal(|| String::new());
    let mut list_signal = use_signal(|| Vec::<Adresse>::new()); 
    
    rsx! {
        h1 { "Hot Dog" }

        div {
            input {  
                r#type: "text",
                value: eingabe,
                oninput: move |e| eingabe.set(e.value()),
            }  
            button {  
                onclick: move |_| async move {
                    let name_to_save = eingabe.read().clone();
                    
                    // HIER: Das Ergebnis loggen
                    match save_name(name_to_save.clone()).await {
                        Ok(id) => {
                            println!("Erfolg! ID: {}", id); // Ausgabe in der Browser-Konsole
                            let adresse = Adresse {
                                id,
                                name: name_to_save,
                            };
                            list_signal.write().push(adresse);
                        }
                        Err(e) => {
                            // WICHTIG: Fehler ausgeben!
                            println!("FEHLER beim Speichern: {:?}", e); 
                            // Optional: Alert im Browser anzeigen (via web-sys oder gloo)
                        }
                    }
                    eingabe.set(String::new());
                },
                disabled: if eingabe.read().trim().is_empty() { true } else { false },
                "save"
            }
        }
        // ... (Rest des Codes für die Liste)
    }
}


#[derive(Debug,Clone,PartialEq, Serialize, Deserialize)]
pub struct Adresse {
    pub id: i64,
    pub name: String,

}


