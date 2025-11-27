use dioxus::prelude::*;
use components::adresse::{list::AdressListe, add::Add};
use components::{home::Home, app_layout::AppLayout};
use crate::backend::models::adresse::Adresse;
use chrono::Local;
use tokio::time::{Duration};



mod backend;
mod components;

// Variablen
static CSS: Asset = asset!("/assets/main.css");
const FAVICON: Asset = asset!("/assets/erp_logo.png");



fn main() {
    dioxus::launch(App);
}

// src/main.rs

#[component]
fn App() -> Element {
    let jetzt = Local::now().format("%H:%M:%S").to_string();

    
    rsx!(
        h1 { "{jetzt}" }
        document::Stylesheet { href: CSS}
        document::Link {rel: "icon", href: FAVICON}
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



