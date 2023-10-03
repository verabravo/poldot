use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;

use serde::Deserialize;
use walkdir::WalkDir;

const CONFIG_PATH: &str = ".config/poldot/config.json";

#[derive(Deserialize, Debug)]
struct Directory {
    alias: String,
    path: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    directories: Vec<Directory>,
}

pub struct ScriptStruct {
    pub alias: String,
    pub module: String,
    pub name: String,
}

pub fn get_scripts() -> Result<Vec<ScriptStruct>, String> {
    let config: Config;
    match get_config() {
        Ok(current_config) => {
            config = current_config;
        }
        Err(err) => return Err(err),
    }
    let mut scripts: Vec<ScriptStruct> = Vec::new();
    for directory in config.directories {
        let directory_path = directory.path;
        let directory_scripts: Vec<ScriptStruct> = get_directory_scripts(directory_path, directory.alias);
        scripts.extend(directory_scripts);
    }
    return Ok(scripts);
}

fn get_directory_scripts(directory: String, alias: String) -> Vec<ScriptStruct> {
    let mut scripts: Vec<ScriptStruct> = Vec::new();
    for entry in WalkDir::new(directory).max_depth(2) {
        match entry {
            Ok(entry) => {
                if entry.file_type().is_dir() {
                    continue;
                }
                let file_name = entry.path().file_name().unwrap().to_str().unwrap();
                if !file_name.ends_with(".sh") {
                    continue;
                }
                let parent_folder_name = entry.path().parent().unwrap().file_name().unwrap().to_str().unwrap().to_string();
                scripts.push(ScriptStruct {
                    alias: alias.clone(),
                    module: parent_folder_name,
                    name: file_name[..file_name.len() - 3].to_string(),
                });
            }
            Err(err) => eprintln!("Error to find the file: {:?}", err),
        }
    }
    return scripts;
}

fn get_config() -> Result<Config, String> {
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