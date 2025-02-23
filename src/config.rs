#[allow(dead_code)]
pub struct Config {
    pub device_name: String,
    pub allow_command_execution: bool,
    pub allow_text_input: bool
}

pub fn get_config() -> Config {
    Config{
        device_name: "nixos".to_owned(),
        allow_command_execution: true,
        allow_text_input: true
    }
}