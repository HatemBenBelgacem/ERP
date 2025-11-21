use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use backend::server_functions::adresse_fns::save_name;

mod backend;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut eingabe = use_signal(|| String::new());
    let mut list_signal = use_context::<Signal<Vec<Adresse>>>();
    
    rsx! {
        h1 { "Hot Dog"}

          div {
            input {  
                r#type:"text",
                value: eingabe,
                oninput: move |e| eingabe.set(e.value()),
            }  
            button {  
                onclick: move |_| async move {
                    match save_name((*eingabe.read()).clone()).await {
                        Ok(id) => {
                            let adresse = Adresse {
                                id,
                                name: (*eingabe.read()).clone(),
                            };
                            let mut new_list = list_signal.read().clone();
                            new_list.push(adresse);
                            list_signal.set(new_list);
                        }
                        Err(_) => {}
                    }
                    eingabe.set(String::new());
                },
                disabled: if eingabe.to_string().trim() == "" {true} else {false},
                "save"
            }
        }
    }
}


#[derive(Debug,Clone,PartialEq, Serialize, Deserialize)]
pub struct Adresse {
    pub id: i64,
    pub name: String,

}


