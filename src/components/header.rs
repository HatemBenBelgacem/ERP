use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx!{
        
        h2 {
            img {src: asset!("/assets/erp_logo.png")}  
            Link{
            to: "/", "ERP"
        }
        }
        

        
    }
}