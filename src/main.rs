use rocket::tokio;
use lazy_static::lazy_static;

mod api;
// mod mdns_resolver;
mod config;

use config::{Config, get_config};
lazy_static! {
    static ref CONFIG: Config = get_config();
}

#[rocket::main]
async fn main() {
    let api_handle = tokio::spawn(api::start());
    // let mdns_resolver_handle = tokio::spawn(mdns_resolver::start());

    let _ = tokio::join!(api_handle);
}