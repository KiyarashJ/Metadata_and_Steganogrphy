
use std::time::Duration;
use dioxus::prelude::*;
use dioxus_timer::use_timer;

use crate::dioxus_components::pub_messages::status_message_handling::Handling;

#[component]
pub fn user_req() -> Element {
    let items: [&'static str; 4] = ["Read metadata", "Delete metadata", "Hide message", "Extract message"];
    let mut main_handling_signal = use_signal(|| Handling::default());
    let main_timer = use_timer(Duration::from_secs(0));

    rsx! {
        div { class: "w-2/3 h-2/3",
            if main_handling_signal.read().req.is_empty() {
                crate::dioxus_components::pub_components::req_ui::req_ui { value: main_handling_signal, items }
            } else {
                crate::dioxus_components::pub_components::upload_file::upload_file {
                    handler: main_handling_signal,
                    on_finished: move || main_handling_signal.write().req = "",
                    main_timer
                }
            }
        }
    }
}