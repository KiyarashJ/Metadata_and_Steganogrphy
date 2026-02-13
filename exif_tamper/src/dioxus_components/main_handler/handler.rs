use std::time::Duration;

use dioxus::prelude::*;
use dioxus_timer::DioxusTimer;

use crate::dioxus_components::pub_messages::status_message_handling::{Handling, Res, Status};


pub fn signal_update_and_retrun_msg (
    handler: Signal<Handling>, 
    main_timer: Signal<DioxusTimer>, 
    status: Status, 
    result: Res, 
    msg: &'static str
    ) {
    let mut handler = handler.clone();
    let mut main_timer = main_timer.clone();
    handler.write().status = status;
    handler.write().result = result;
    handler.write().message = msg;
    main_timer.write().set_preset_time(Duration::from_secs(5));
    main_timer.write().start();
}