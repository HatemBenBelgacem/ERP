use dioxus::prelude::*;
use crate::icons::Icon;
use icons::mdi::Home;

use crate::backend::server_functions::adresse_fns::delete_adresse;
use crate::backend::models::adresse::Adresse;
 

#[component]
pub fn Delete(mut adresse_resource: Resource<Result<Vec<Adresse>, ServerFnError>>, id:i64) -> Element {
    rsx!{
        button { 
            onclick: move |_| async move {
                match delete_adresse(id).await {
                    Ok(_) => {
                        adresse_resource.restart();
                    }
                    Err(e) => {}
                }
            },
            "löschen"
        }
    }
}