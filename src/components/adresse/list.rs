use dioxus::prelude::*;

use crate::{Route, Adresse};


#[component]
pub fn List () -> Element {
  let list = use_context::<Signal<Vec<Adresse>>>();
  
  if list.read().len() == 0 {
    rsx!(
      div { class: "text-center text-xl", "No ToDos" }
    )
  } else {
      rsx!(
        div { class: "h-screen w-full flex justify-center ",
          ul { class: "sm:w-1/2 w-5/6",
            {
                list.read()
                    .iter()
                    .map(|item| {
                        let id = item.id;
                        rsx! {
                          li {
                            class: "mb-2 flex justify-between items-center bg-slate-100  px-2 py-2 rounded hover:bg-slate-200",
                            key: id,
                            div { class: "flex",
                              Link { to: Route::Adresse { id }, "{item.vorname.clone()}" }
                            }
                           
                          }
                        }
                    })
            }
          }
        }
      )
  }
}