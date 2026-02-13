
use std::time::Duration;

use dioxus::{prelude::*};
use dioxus_timer::{DioxusTimer, TimerState};
use crate::{dioxus_components::pub_messages::status_message_handling::{Handling, Status}, hide_msg_in_img::encode::encode_img};

#[derive(Clone, PartialEq, Default)]
pub struct Encode {
    msg: String,
    passphrase: String,
    cr: std::path::PathBuf
}


#[component]
pub fn hide (
    file: std::path::PathBuf, 
    on_finished: Callback<()>,
    handler: Signal<Handling>,
    main_timer: Signal<DioxusTimer>
    ) -> Element {
    let mut form_state = use_signal::<Encode>(|| Encode::default());
    use_effect(move || {
        if main_timer.read().state() == TimerState::Finished {
            on_finished.call(());
            main_timer.write().reset();
        }

    });
       rsx! {
        h1 { "Write your massage to be hidden:" }
        input {
            class: "w-2/3 h-1/6 opacity-50 border-4 rounded-lg border-white cursor-pointer hover:scale-101 hover:transition-all hover:ease-in hover:duration-150 hover:bg-[rgba(255,255,255,0.2)] mb-10",
            r#type: "text",
            value: "{form_state.read().msg}",
            oninput: move |e| form_state.write().msg = e.value(),
        }
        h1 { "Write your passphrase:" }
        input {
            class: "w-2/3 h-1/6 opacity-50 border-4 rounded-lg border-white cursor-pointer hover:scale-101 hover:transition-all hover:ease-in hover:duration-150 hover:bg-[rgba(255,255,255,0.2)] mb-10",
            r#type: "password",
            value: "{form_state.read().passphrase}",
            oninput: move |e| form_state.write().passphrase = e.value(),
        }
        button {
            class: "mt-10 text-slate-950 rounded-sm ml-10 text-xl w-30 h-10 bg-[rgba(255,255,255,1)] cursor-pointer hover:rounded-lg hover:transition-all hover:ease-in hover:duration-150 hover:scale-105 hover:bg-orange-200",
            onclick: move |_| {
                let data = form_state.read();
                let _ = match encode_img(&file, &data.msg, &data.passphrase) {
                    "done" => {
                        handler.write().status = crate::dioxus_components::pub_messages::status_message_handling::Status::MessageEncodedSuccessfully;
                        main_timer.write().set_preset_time(Duration::from_secs(6));
                        main_timer.write().start();
                    }, 
                    _ => {handler.write().status = crate::dioxus_components::pub_messages::status_message_handling::Status::ErrorInEncodingMessage}
                };
            },
            "Done"
        }
        if handler.read().status == Status::Def {
            if handler.read().status == Status::MessageEncodedSuccessfully {
                crate::dioxus_components::pub_messages::success::success { message: handler.read().message }
            } else {
                crate::dioxus_components::pub_messages::fail::fail { message: handler.read().message }
            }
        }
    }        
}