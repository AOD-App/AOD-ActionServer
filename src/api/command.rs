use std::process::Command;

use rocket::post;
use rocket::serde::{Deserialize, json::Json};

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct CommandTemplate {
    cmd: String,
    args: Vec<String>,
}
#[post("/command/execute", data="<command>")]
pub async fn execute(command: Json<CommandTemplate>) {
    Command::new(&command.cmd)
        .args(&command.args)
        .output()
        .unwrap();
}