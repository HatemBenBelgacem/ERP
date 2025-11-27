use dioxus::prelude::*;
use components::adresse::{list::AdressListe, add::Add};
use components::{home::Home, app_layout::AppLayout};
use crate::backend::models::adresse::Adresse;



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



