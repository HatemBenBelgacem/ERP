use dioxus::prelude::*;

use crate::backend::server_functions::adresse_fns::add_new_adresse;
use crate::Adresse;


#[component]
pub fn Add() -> Element {
  let mut content = use_signal(|| String::new());
  let mut list_signal = use_context::<Signal<Vec<Adresse>>>();

  rsx!(
    div { class: "flex w-full justify-center my-2",
      div { class: "sm:w-1/2 w-5/6",
        input {
          class: "w-3/4 p-1 rounded border-2 border-slate-100 border-solid",
          r#type: "text",
          value: content,
          oninput: move |e| content.set(e.value()),
        }
        button {
          class: "text-slate-50 bg-sky-500 p-1 rounded w-1/4 hover:bg-sky-600",
          onclick: move |_| async move {
              match add_new_adresse(*content.read().clone()).await {
                  Ok(id) => {
                      let adresse = Adresse {
                          id,
                          vorname: (*content.read()).clone(),
                          nachname: (*content.read()).clone()
                      };
                      let mut new_list = list_signal.read().clone();
                      new_list.push(adresse);
                      list_signal.set(new_list);
                  }
                  Err(_) => {}
              }
              content.set(String::new());
          },
          disabled: if content.to_string().trim() == "" { true } else { false },
          "add"
        }
      }
    }
  )
}