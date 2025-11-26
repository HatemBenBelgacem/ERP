use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx!{
        h3 {  
            Link{
            to: "/", "ERP"
        }
        }
        

        
    }
}