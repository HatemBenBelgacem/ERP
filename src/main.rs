use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use components::{home::Home, list::AdressListe, app_layout::AppLayout, add::Add};


mod backend;
mod components;

// Variablen
static CSS: Asset = asset!("/assets/main.css");



fn main() {
    dioxus::launch(App);
}

// src/main.rs

#[component]
fn App() -> Element {
    rsx!(
        document::Stylesheet { href: CSS}
        Router::<Route> {}
    )
        
}

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[layout(AppLayout)]
    #[route("/")]
    Home {},
    #[route("/adressen")]
    AdressListe {},
    #[route("/adressen/add")]
    Add{}
}


#[derive(Debug,Clone,PartialEq, Serialize, Deserialize)]
pub struct Adresse {
    pub id: i64,
    pub vorname: String,
    pub nachname: String,
}


