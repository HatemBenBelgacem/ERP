
use dioxus::prelude::*;
use crate::{backend::server_functions::auftrag_fns::save_auftrag, Auftrag};
use crate::{backend::server_functions::{adresse_fns::adress_liste}};

#[component]
pub fn AddAuftrag() -> Element {
    let mut bezeichnung = use_signal(|| String::new());
    let mut selected_adresse_id = use_signal(|| 0i64);
    let mut list_signal = use_signal(|| Vec::<Auftrag>::new()); 

    let nav = use_navigator();

    let adressen_resource = use_resource(move || async move {
        adress_liste().await
    });
    
    rsx! {
        div { class:"add_form",
                label { "Aktiv" }
                input {  class:"check",
                    r#type: "checkbox"
                }
                br {  }
                
                label { "Bezeichnung" }
                br {  }
                input {  
                    r#type: "text",
                    value: bezeichnung,
                    oninput: move |e| bezeichnung.set(e.value()),
                }
                br {  }
                
                label { "Kunde auswählen" }
                br {  }

                // 2. Dropdown Menü (Select)
                match &*adressen_resource.read_unchecked() {
                    Some(Ok(adressen)) => rsx! {
                        select {
                            // Wenn sich die Auswahl ändert, aktualisieren wir das Signal
                            onchange: move |evt| {
                                if let Ok(id) = evt.value().parse::<i64>() {
                                    selected_adresse_id.set(id);
                                }
                            },
                            style: "width: 50%; height: 35px; margin-bottom: 10px; border-radius: 5px; border: 0.5px solid grey;",
                            
                            // Standard Option
                            option { value: "0", "Bitte wählen..." }

                            // 3. Iteration über die geladenen Adressen
                            for adresse in adressen {
                                option { 
                                    value: "{adresse.id}", 
                                    "{adresse.vorname} {adresse.nachname}" 
                                }
                            }
                        }
                    },
                    Some(Err(e)) => rsx! { "Fehler beim Laden der Kunden: {e}" },
                    None => rsx! { "Lade Kunden..." }
                }
                
                br {  }

                button {  class:"btn",
                    onclick: move |_| async move {
                        let save_bezeichnung = bezeichnung.read().clone();
                        let save_id = *selected_adresse_id.read(); // ID auslesen
                        
                        // Validierung: Nur speichern, wenn eine ID > 0 gewählt wurde
                        if save_id > 0 {
                            match save_auftrag(save_bezeichnung.clone(), save_id).await {
                                Ok(id) => {
                                    println!("Erfolg! ID: {}", id);
                                    nav.push("/auftrag");
                                }
                                Err(e) => {
                                    println!("FEHLER: {:?}", e); 
                                }
                            }
                            bezeichnung.set(String::new());
                            selected_adresse_id.set(0);
                        } else {
                            println!("Bitte einen Kunden auswählen!");
                        }
                    },
                    // Button deaktivieren, wenn Text leer oder kein Kunde gewählt (ID 0)
                    disabled: bezeichnung.read().trim().is_empty() || *selected_adresse_id.read() == 0,
                    "Speichern"
                }
        }
        Link { 
            class:"btn",
            to: "/auftrag", "Abbrechen" 
        }
    }
}



