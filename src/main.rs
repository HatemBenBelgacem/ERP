use dioxus::prelude::*;
use components::adresse::{list::AdressListe, add_adresse::AddAdresse, detail_adresse::AdresseDetail};
use components::auftrag::{add_auftrag::AddAuftrag, auftrag::AuftragListe};
use components::{home::Home, app_layout::AppLayout};
use crate::backend::models::adresse::Adresse;
use crate::backend::models::auftrag::Auftrag;




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
    

    
    rsx!(
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
    #[route("/adresse/:id")]
    AdresseDetail{id:i64},
    #[route("/adressen/add")]
    AddAdresse{},
    #[route("/auftrag")]
    AuftragListe{},
    #[route("/auftrag/add")]
    AddAuftrag{}
}   




