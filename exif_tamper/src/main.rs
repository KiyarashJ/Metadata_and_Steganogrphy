use dioxus::prelude::*;


pub mod read_n_delete;
pub mod dioxus_components;
pub mod hide_msg_in_img;





const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    let _ = match rayon::ThreadPoolBuilder::new().build_global() {
        Ok(_) => println!("threadpool is done successfully"),
        Err(e) => eprintln!("An Error occured : {}", e)
    };
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Ui {}

    }
}

#[component]
pub fn Ui() -> Element {
    rsx! {
        div { class: "w-full h-screen bg-linear-to-tr from-gray-950 to-violet-950",
            h1 { class: "text-5xl text-center pt-5 text-[rgba(255,255,255,0.9)]",
                "Exif meta Data Tampering"
            }
            div { class: "size-full flex justify-center items-center",
                crate::dioxus_components::pub_components::user_req::user_req {}
            }
        }
    }
}





