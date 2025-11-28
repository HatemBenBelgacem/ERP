use dioxus::prelude::*;

use crate::backend::server_functions::adresse_fns::delete_adresse;
use crate::backend::models::adresse::Adresse;

#[component]
pub fn Delete(list_signal: Signal<Vec<Adresse>>, id:i64) -> Element {
    rsx!{
        button {
            onclick: move |_| async move {
                match delete_adresse(id).await {
                    Ok(_) => {
                        let new_list = list_signal
                            .read()
                            .iter()
                            .filter(|item| item.id != id)
                            .map(|item| item.clone())
                            .collect::<Vec<Adresse>>();
                        list_signal.set(new_list);
                    }
                    Err(_) => {}
                }
            },
            "löschen"
        }
    }
}