
use dioxus::prelude::*;
use dioxus_timer::DioxusTimer;

use crate::dioxus_components::pub_messages::status_message_handling::Handling;






#[component]
pub fn get_infos(file: std::path::PathBuf, on_finished: Callback<()>, handler : Signal<Handling>, main_timer: Signal<DioxusTimer>) -> Element {
    
    rsx!{
        {
            if handler.read().req.to_lowercase() == "hide message" {
                rsx! {
                    crate::dioxus_components::steganography::hide::hide { file , on_finished, handler, main_timer }
                }
            } else {
                rsx! {
                    crate::dioxus_components::steganography::extract::extract { file , on_finished, handler, main_timer }
                }
            }
        }
    }
}
