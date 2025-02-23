use rocket::{get, routes};
use rocket::serde::{json::Json, Serialize};
use local_ip_address::local_ip;
use enigo::{Enigo, Settings};
use std::sync::{Arc, Mutex};

use crate::CONFIG;

mod keyevent;
mod mouseevent;
mod notification;
mod command;
mod settings;

pub async fn start() {

    let enigo = Arc::new(Mutex::new(Enigo::new(&Settings::default()).unwrap()));

    let _rocket = rocket::build()
        .mount("/", routes![
            index,
            status,
            mouseevent::left_click,
            mouseevent::right_click,
            keyevent::left,
            keyevent::right,
            // keyevent::volume_up,
            // keyevent::volume_down,
            notification::notify,
            command::execute,
            settings::settings_get,
            settings::settings_post,
        ])
        .manage(enigo)
        .launch()
        .await;
}

#[get("/")]
async fn index() -> &'static str {
    "AOD (Android on Desktop)"
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Status {
    device: String,
    ip: String
}

#[get("/status")]
async fn status() -> Json<Status> {
    Json(Status {
        device: CONFIG.device_name.clone(),
        ip: local_ip().unwrap().to_string()
    })
}
