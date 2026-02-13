
use dioxus::prelude::*;
use dioxus_timer::{DioxusTimer, TimerState};
use crate::{dioxus_components::{main_handler::handler::signal_update_and_retrun_msg, pub_messages::status_message_handling::{Handling, Res, Status}}, hide_msg_in_img::decode::decode_img};

#[derive(Clone, Default, PartialEq)]
pub struct Decode {
    code: String,
    secret: String,
    passphrase: String,
    cr: std::path::PathBuf
}



#[component]
pub fn extract(
    file : std::path::PathBuf, 
    on_finished: Callback<()>, 
    handler: Signal<Handling>,
    main_timer: Signal<DioxusTimer>
) -> Element {
    let mut decode = use_signal(|| Decode::default());
    use_effect(move || {
        if main_timer.read().state() == TimerState::Finished {
            on_finished.call(());
            main_timer.write().reset();
        }
    });

     
                rsx! {
                    h1 { "Write your code to be decoded: " }
                    input {
                        class: "w-full h-10 rounded-lg bg-[rgba(255,255,255,0.1)] text-white px-4 py-2 mb-4",
                        r#type: "text",
                        value: "{decode.read().code}",
                        oninput: move |e| { decode.write().code = e.value() },
                        "this is for your msg to hide or extract"
                    }
                    h1 { "Write your passphrase: " }
                    input {
                        class: "w-full h-10 rounded-lg bg-[rgba(255,255,255,0.1)] text-white px-4 py-2 mb-4",
                        r#type: "password",
                        value: "{decode.read().passphrase}",
                        oninput: move |e| { decode.write().passphrase = e.value() },
                        "this is for your passphrase"
                    }
                    h1 { "Write your secret key: " }
                    input {
                        class: "w-full h-10 rounded-lg bg-[rgba(255,255,255,0.1)] text-white px-4 py-2 mb-4",
                        r#type: "text",
                        value: "{decode.read().secret}",
                        oninput: move |e| { decode.write().secret = e.value() },
                        "this is for your passphrase"
                    }
                    button {
                        class: "mt-10 text-slate-950 rounded-sm ml-10 text-xl w-20 h-10 bg-[rgba(255,255,255,1)] cursor-pointer hover:rounded-lg hover:transition-all hover:ease-in hover:duration-150 hover:scale-105 hover:bg-orange-200",
                        onclick: move |_| {
                            let data = decode.read();
                            let _ = match decode_img(&file, &data.passphrase, &data.code, data.secret.to_string()) {
                                "done" => {
                                    let _ = signal_update_and_retrun_msg (
                                            handler, 
                                            main_timer, 
                                            crate::dioxus_components::pub_messages::status_message_handling::Status::MessageDecodedSuccessfully, 
                                            Res::Success("successfully extracted hidden message"), 
                                            "file's hidden message extracted successfully"
                                    );
                                }, 
                                _ => {
                                    let _ = signal_update_and_retrun_msg (
                                            handler, 
                                            main_timer, 
                                            crate::dioxus_components::pub_messages::status_message_handling::Status::ErrorInDecodingMessage, 
                                            Res::Fail("error while extracting hidden message"), 
                                            "while extracting an error occured"
                                    );
                                }
                            };
                        },
                        "done"
                    }
                    
                    if handler.read().status != Status::Def {
                        crate::dioxus_components::pub_messages::success::success { message: handler.read().message }
                    }
                }
}