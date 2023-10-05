use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;

use serde::Deserialize;

const CONFIG_PATH: &str = ".config/poldot/config.json";

#[derive(Deserialize, Debug, Clone)]
pub struct Directory {
    pub alias: String,
    pub path: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub directories: Vec<Directory>,
}

pub fn get_config() -> Result<Config, String> {
    let home_dir: OsString;
    match env::var_os("HOME") {
        Some(val) => home_dir = val,
        None => return Err("$HOME is undefined".to_string())
    }
    let filename = format!("{}/{}", home_dir.to_str().unwrap(), CONFIG_PATH);
    let mut file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => return Err(format!("Config file not found: {}", err))
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Err(_) => return Err("Config file is empty".to_string()),
        _ => {}
    }

    return match serde_json::from_str(&contents) {
        Ok(config) => Ok(config),
        Err(err) => Err(format!("Error parsing config file, json is not valid: {}", err))
    }
}