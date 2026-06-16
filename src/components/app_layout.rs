use dioxus::prelude::*;

use crate::components::{header::Header, nav::Nav};
use crate::Route;

#[component]
pub fn AppLayout() -> Element {
    rsx! {
        // Ein Container, der die gesamte Höhe des Bildschirms einnimmt (min-vh-100)
        // und Elemente als Spalte untereinander anordnet (flex-column)
        div { class: "d-flex flex-column min-vh-100",

            // Ganz oben ist der Header in voller Breite
            Header {}

            // Der Hauptbereich (container-fluid = volle Breite, flex-grow-1 = füllt den restlichen Platz)
            div { class: "container-fluid flex-grow-1 d-flex flex-column",

                // Eine Bootstrap-Row (Reihe) für das Grid-System
                div { class: "row flex-grow-1",

                    // Linke Spalte für das Menü (2 von 12 Spalten)
                    // "bg-light" für hellgrauen Hintergrund, "border-end" für eine Trennlinie rechts
                    div { class: "col-md-2 bg-light border-end p-3", Nav {} }

                    // Rechte Spalte für den Inhalt (10 von 12 Spalten)
                    // "p-4" gibt dem Inhalt einen großzügigen inneren Abstand (Padding)
                    div { class: "col-md-10 p-4", Outlet::<Route> {} }
                }
            }
        }
    }
}