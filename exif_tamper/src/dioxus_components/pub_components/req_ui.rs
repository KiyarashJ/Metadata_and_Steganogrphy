
use dioxus::prelude::*;

use crate::dioxus_components::pub_messages::status_message_handling::Handling;


#[component]
pub fn req_ui(value: Signal<Handling>, items: [&'static str; 4]) -> Element{
    rsx!{

        div { class: "size-full",
            h1 { class: "text-2xl", "What do you want to do with your image ?" }
            div { class: "flex justify-center items-center flex-wrap gap-4",
                {
                    items
                        .iter()
                        .map(|&item| {
                            let item_value = item as &'static str;
                            rsx! {
                                button {
                                    class: " mt-10 text-slate-950 rounded-sm ml-10 text-xl min-w-1/5 min-h-10 bg-[rgba(255,255,255,1)] cursor-pointer hover:rounded-lg hover:transition-all hover:ease-in hover:duration-150 hover:scale-105 hover:bg-orange-400",
                                    onclick: move |_| {
                                        value.write().req = item_value;
                                        value.with_mut(|h| {
                                            h.req = item_value;
                                        })
                                    },
                                    "{item}"
                            }
                        }
                        })
                }
            }
        }

    }
}