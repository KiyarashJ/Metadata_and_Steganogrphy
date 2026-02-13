use dioxus::prelude::*;
use chrono::Utc;
use crate::dioxus_components::main_handler::handler::signal_update_and_retrun_msg;
use crate::dioxus_components::pub_messages::status_message_handling::{Res, Status};
use crate::{dioxus_components::pub_messages::status_message_handling::Handling, read_n_delete::read::read};
use crate::read_n_delete::delete::delete; 
use rayon::prelude::*;
use dioxus_timer::{DioxusTimer, TimerState};  



#[component]
pub fn upload_file(
    handler: Signal<Handling>,
    on_finished: Callback<()>,
    main_timer: Signal<DioxusTimer>
) -> Element {
    let mut selected_file = use_signal(|| None::<std::path::PathBuf>);

    use_effect(move || {
        if main_timer.read().state() == TimerState::Finished {
            on_finished.call(());
            main_timer.write().reset(); 
        }
    });

    let req_lower = handler.read().req.to_lowercase();
    rsx! {
        div { class: "w-full h-2/3 rounded-lg",
            {
                if selected_file.read().is_none() {
                    rsx! {
                        input {
                            class: "w-full h-full opacity-50 border-4 rounded-lg border-white cursor-pointer hover:scale-101 hover:transition-all hover:ease-in hover:duration-150 hover:bg-[rgba(255,255,255,0.2)] mb-10",
                            r#type: "file",
                            accept: ".jpg,.jpeg,.png",
                            multiple: true,
                            onchange: move |event: Event<FormData>| {
                                let files = event.files();

                                let file_infos: Vec<(std::path::PathBuf, String)> = files
                                    .into_par_iter()
                                    .map(|f| (f.path(), f.name()))
                                    .collect();
                                
                                
                                if req_lower.contains("read metadata") || req_lower.contains("delete metadata") {
                                    handler.write().status = crate::dioxus_components::pub_messages::status_message_handling::Status::ProcessingMetaDataFiles;
                                    
                                    let start = Utc::now();
                                    let copy = if handler.read().copy == 1 {
                                        1
                                    } else {
                                        0
                                    };

                                    let results: Vec<Result<(), _>> = file_infos
                                        .par_iter()
                                        .map(|(path, name)| {
                                            if req_lower.contains("read metadata") {
                                                read(path, &name)
                                            } else {
                                                delete(&path, &name, copy)
                                            }
                                        })
                                        .collect();

                                    let end = Utc::now();
                                    let duration_ms = (end - start).num_milliseconds();
                                    println!("\n\nDuration: {} milliseconds\n\n", duration_ms);

                                    let success_count = results.iter().filter(|r| r.is_ok()).count();
                                    let total = results.len();

                                    if success_count == total {
                                        let _ = signal_update_and_retrun_msg(
                                            handler, 
                                            main_timer, 
                                            crate::dioxus_components::pub_messages::status_message_handling::Status::AllFilesExifReadSuccessfully, 
                                            Res::Success("successfully proccessed files"), 
                                            "proccessed successfully all files"
                                        );
                                    } else {
                                        let _ = signal_update_and_retrun_msg(
                                            handler, 
                                            main_timer, 
                                            crate::dioxus_components::pub_messages::status_message_handling::Status::SomeFilesExifReadSuccessfully, 
                                            Res::Success("successfully processed some files"), 
                                            "some files are dropped or have no exif but the others proccessed successfully"
                                        );
                                    }
                                } else {
                                    if let Some((path, _name)) = file_infos.into_iter().next() {
                                        selected_file.set(Some(path));
                                    }
                                }
                            },
                        }
                        
                        if req_lower == "delete metadata" { 
                            h1 { 
                            class: "text-lg",
                            "do you wanna copy from a file when deleting metadata ?"
                            }
                            input { 
                                r#type: "checkbox",
                                oninput: move |e| {
                                    if e.data().value() == "true" {
                                        handler.write().copy = 1
                                    } else {
                                        handler.write().copy = 0
                                    }
                                }
                            }
                        }
                        if handler.read().status != Status::Def {
                            if (
                                    handler.read().result == Res::Success("successfully processed some files") || handler.read().result == Res::Success("successfully processed files"))
                                    && 
                                    (handler.read().status == Status::AllFilesExifReadSuccessfully || handler.read().status == Status::SomeFilesExifReadSuccessfully
                                ) 
                            {
                                crate::dioxus_components::pub_messages::success::success { message: handler.read().message }
                            } else {
                                crate::dioxus_components::pub_messages::pending::pending { message: handler.read().message }
                            }
                        }
                    }
                } else {
                    if let Some(file) = selected_file()
                        && (req_lower.to_lowercase().contains("hide message")
                            || req_lower.to_lowercase().contains("extract message"))
                    {
                        rsx! {
                            crate::dioxus_components::steganography::get_info::get_infos { file , on_finished, handler, main_timer }
                        }
                    } else {
                        if handler.read().status == Status::Def {
                            rsx!{
                                crate::dioxus_components::pub_messages::fail::fail { message: handler.read().message }
                            }
                        } else {
                            rsx!{
                                crate::dioxus_components::pub_messages::success::success { message : handler.read().message }
                            }
                        }
                    }
                }
            }
        }
    }
}
