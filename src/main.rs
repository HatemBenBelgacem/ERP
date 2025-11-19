use dioxus::prelude::*;
use serde::{Deserialize, Serialize};


use crate::backend::server_functions::adresse_fns::get_adresse_liste;

mod backend;
mod components;

use components::adresse::{detail::Detail, home::Home};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/adresse/:id")]
    Detail { id: i64 },
}


fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut list = use_signal(|| vec![]);

    let _ = use_resource(move || async move {
        match get_adresse_liste().await {
            Ok(adresse) => list.set(adresse),
            Err(_) => ()
        }
    });
    
    rsx! {
        Router::<Route> {}
    }
}


#[derive(Debug,Clone,PartialEq, Serialize, Deserialize)]
pub struct Adresse {
    pub id: i64,
    pub vorname: String,

}


