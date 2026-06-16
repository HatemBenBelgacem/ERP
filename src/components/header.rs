use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx! {
        // "navbar" ist die Grundklasse. "navbar-dark bg-dark" macht sie dunkel.
        // Du kannst auch "navbar-light bg-light" oder "bg-primary" (blau) nutzen.
        nav { class: "navbar navbar-expand-lg navbar-dark bg-dark",
            div { class: "container-fluid", // Nimmt die volle Breite ein

                // "navbar-brand" ist speziell für das Logo und den Titel gedacht
                Link { class: "navbar-brand d-flex align-items-center", to: "/",

                    img {
                        src: asset!("/assets/erp_logo.png"),
                        width: "30", // Feste Größe für das Logo im Header
                        height: "30",
                        class: "me-2", // "me-2" (margin-end) sorgt für etwas Abstand rechts zum Text
                    }
                    "ERP System"
                }
            }
        }
    }
}