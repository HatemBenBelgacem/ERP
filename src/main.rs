use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use components::{add::Add, home::Home};

mod backend;
mod components;

fn main() {
    dioxus::launch(App);
}

// src/main.rs

#[component]
fn App() -> Element {
    rsx!(
        Router::<Route> {}
    )
        
}

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/add")]
    Add{}
}


#[derive(Debug,Clone,PartialEq, Serialize, Deserialize)]
pub struct Adresse {
    pub id: i64,
    pub vorname: String,
    pub nachname: String,
}


