
use dioxus::prelude::*;
use tokio::time::{sleep, Duration};

#[component]
pub fn msg(message: &'static str) -> Element {
    rsx! {
        div {
            h1 { class: "text-green-500 text-2xl mt-10", "{message}" }
        }
    }
}





#[component]
pub fn success(message: &'static str) -> Element {
    let mut is_shown = use_signal(|| true);
    use_future(move || async move {
        sleep(Duration::from_secs(5)).await;
        is_shown.set(false);
    });
    rsx!{
        div {
            {
                if is_shown() {
                    rsx! {
                        crate::dioxus_components::pub_messages::success::msg { message }

                    }
                } else {
                    rsx! {
                        crate::dioxus_components::pub_components::user_req::user_req {}
                    }
                }
            }
        }
    }
}

