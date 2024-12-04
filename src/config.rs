use serde::Deserialize;
use toml;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub bind_address: String,
    pub custom_css: String,
    pub bar: Option<Bar>,
}

#[derive(Deserialize)]
#[derive(Clone)]
pub struct Bar {
    pub names: Vec<String>,
    pub urls: Vec<String>,
}

pub fn open_config() -> Config {

    let file = match fs::read_to_string("config.toml") {
        Ok(val) => val,
        Err(_) => panic!("Could not open config. Does file exist/is readable?")
    };

    let config: Config = match toml::from_str(file.as_str()) {
        Ok(val) => val,
        Err(_) => panic!("Could not parse config.toml. Check formatting?")
    };

    config
}