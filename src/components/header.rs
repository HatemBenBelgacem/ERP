use dioxus::prelude::*;
use chrono::Local;


#[component]
pub fn Header() -> Element {
    let jetzt = Local::now().format("%H:%M:%S").to_string();
    rsx!{
        
        h2 {
            img {src: asset!("/assets/erp_logo.png")}  
            Link{
                to: "/", "ERP ",
                p { "Aktuelle Zeit: ", "{jetzt}" }
            }
        }
        
        
    }
}