use rocket::{get, post};
use rocket::serde::json::Json;
use crate::config::{get_config, set_config, Config};

#[get("/settings")]
pub async fn settings_get() -> Json<Config> {
    Json(get_config())
}

#[post("/settings", data="<new_settings>")]
pub async fn settings_post(new_settings: Json<Config>) {
    set_config(new_settings.into_inner());
}