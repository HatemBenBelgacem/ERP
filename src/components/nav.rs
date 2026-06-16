use dioxus::prelude::*;

#[component]
pub fn Nav() -> Element {
    rsx! {
        div { class: "d-flex flex-column",
            h5 { class: "mt-3 mb-3 text-uppercase text-secondary" }

            // "nav flex-column" ordnet die Links untereinander an
            // "nav-pills" gibt den Links bei Bedarf einen schönen Hintergrund, wenn sie aktiv sind
            ul { class: "nav flex-column nav-pills",
                li { class: "nav-item mb-1",
                    Link { class: "nav-link text-dark", to: "/adressen", "Adressen" }
                }
                li { class: "nav-item mb-1",
                    Link { class: "nav-link text-dark", to: "/auftrag", "Aufträge" }
                }
            }
        }
    }
}