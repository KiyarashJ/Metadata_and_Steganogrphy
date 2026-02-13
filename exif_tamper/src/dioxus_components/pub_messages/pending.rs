
use dioxus::prelude::*;

#[component]
pub fn pending(message: &'static str) -> Element {
    rsx! {
        h1 { class: "text-yellow-500 text-2xl", "{message}" }
    }
}
