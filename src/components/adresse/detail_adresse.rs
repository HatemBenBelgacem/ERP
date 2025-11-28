use dioxus::prelude::*;

use crate::backend::server_functions::adresse_fns::detail_adresse;
use crate::Route;


#[component]
pub fn AdresseDatail(id: i64) -> Element {
    let adresse_resource = use_resource(move || async move {
        detail_adresse(id).await
    });

    rsx! {
        div {
            class: "container",
            match &adresse_resource.read_unchecked() {
                Some(Ok(adresse)) => rsx!{
                    h2 { "Details: {adresse.vorname} {adresse.nachname}" }
                    p { "ID: {adresse.id}" }
                    p { "Strasse: {adresse.strasse} {adresse.strassen_nr}" }

                    Link { 
                        class: "btn", 
                        to: Route::AdressListe {}, 
                        "Zurück zur Liste" 
                    }
                },
                Some(Err(e)) => rsx! { div { color: "red", "Fehler: {e}" } },
                None => rsx! { "Lade Details..." }
            }
        }
    }
}