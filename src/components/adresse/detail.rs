use dioxus::prelude::*;

use crate::backend::server_functions::adresse_fns::get_single_adresse;

#[component]
pub fn Detail(id: i64) -> Element {
  let mut list = use_signal(|| vec![]);
  let navigator = use_navigator();
  let _ = use_resource(move || async move {
    match get_single_adresse(id).await {
      Ok(adresse) => list.set(vec![adresse]),
      Err(_) => ()
    }
  });

  if list.read().len() != 0 {
    rsx!(
      div { class: "my-5 flex justify-center",
        div { class: "border-solid border-slate-100 border-2 rounded p-1",
          button {
            class: "bg-slate-50 rounded p-1 hover:bg-slate-100",
            onclick: move |_| navigator.go_back(),
            "go back"
          }
          div { class: "text-3xl",
            h1 { "id: {list.read()[0].id}" }
            h1 { "content: {list.read()[0].vorname}" }

          }
        }
      }
    )
  } else {
    rsx!(
      div { class: "my-5 flex justify-center",
        button {
          class: "bg-slate-50 rounded p-1 hover:bg-slate-100",
          onclick: move |_| navigator.go_back(),
          "go back"
        }
        div { class: "text-5xl", "To-Do id : {id} Not Found!" }
      }
    )
  }
}