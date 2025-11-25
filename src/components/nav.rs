use dioxus::prelude::*;



#[component]
pub fn Nav() -> Element {
    rsx!{
        h1 {  "Navigation"}

        li { 
            ul {
                class:"nav", 
                Link { to: "/adressen", "Adressen" }
            }
        }    
       
    }
}