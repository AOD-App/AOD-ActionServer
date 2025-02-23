use toml;
use std::fs;
use serde::{Serialize, Deserialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub device_name: String,
    pub allow_command_execution: bool,
    pub allow_text_input: bool
}

const LINUX_CONFIG_LOCATION: &'static str = "/home/mathew/.config/aod/settings.toml";

pub fn get_config() -> Config {

    let data = fs::read_to_string(LINUX_CONFIG_LOCATION)
        .expect("Unable to read settings.toml");
    
    let config: Config = toml::from_str(&data)
        .expect("Invalid sequence in settings.toml");

    config
}

pub fn set_config(config: Config) {

    let data = toml::to_string(&config)
        .expect("Invalid settings schema");

    fs::write(LINUX_CONFIG_LOCATION, data).unwrap()
}