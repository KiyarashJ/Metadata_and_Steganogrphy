
use dioxus::prelude::*;
#[component]
pub fn fail(message: &'static str) -> Element {
    rsx!{
        div {
            h1 { class: "text-red-500 text-2xl", "{message}" }
        }
    }
}