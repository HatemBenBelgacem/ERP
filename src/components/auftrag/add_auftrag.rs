
use dioxus::prelude::*;
use crate::{backend::server_functions::auftrag_fns::save_auftrag, Auftrag};

#[component]
pub fn AddAuftrag() -> Element {
    let mut bezeichnung = use_signal(|| String::new());
    let mut kunde = use_signal(|| String::new());
    let mut list_signal = use_signal(|| Vec::<Auftrag>::new()); 

    let nav = use_navigator();
    
    rsx! {
        div { class:"add_form",
                label { "Bezeichnung" }
                br {  }
                input {  
                r#type: "text",
                value: bezeichnung,
                oninput: move |e| bezeichnung.set(e.value()),
                }
                br {  }
                label { "Kunde" }
                br {  }
                input {  
                    r#type: "text",
                    value: kunde,
                    oninput: move |e| kunde.set(e.value()),
                }
                br {  }

                button {  class:"btn",
                    onclick: move |_| async move {
                        let save_bezeichnung = bezeichnung.read().clone();
                        let save_kunde = kunde.read().clone();
                        
                        // HIER: Das Ergebnis loggen
                        match save_auftrag(save_bezeichnung.clone(), save_kunde.clone()).await {
                            Ok(id) => {
                                println!("Erfolg! ID: {}", id); // Ausgabe in der Browser-Konsole
                                let auftrag = Auftrag {
                                    id,
                                    bezeichnung: save_bezeichnung,
                                    kunde: save_kunde,

                                };
                                list_signal.write().push(auftrag);
                                nav.push("/adressen");
                            }
                            Err(e) => {
                                // WICHTIG: Fehler ausgeben!
                                println!("FEHLER beim Speichern: {:?}", e); 
                                // Optional: Alert im Browser anzeigen (via web-sys oder gloo)
                            }
                        }
                        bezeichnung.set(String::new());
                        kunde.set(String::new());

                        
                    },
                    disabled: if bezeichnung.read().trim().is_empty() || kunde.read().trim().is_empty() { true } else { false },
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


