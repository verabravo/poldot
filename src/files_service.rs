use walkdir::WalkDir;

use crate::config_service::{Config, get_config};

#[derive(Clone)]
pub struct ScriptStruct {
    pub alias: String,
    pub module: String,
    pub name: String,
}

pub fn get_scripts(config: Config) -> Result<Vec<ScriptStruct>, String> {
    let mut scripts: Vec<ScriptStruct> = Vec::new();
    for directory in config.directories {
        let directory_path = directory.path;
        let directory_scripts: Vec<ScriptStruct> = get_directory_scripts(directory_path, directory.alias);
        scripts.extend(directory_scripts);
    }
    return Ok(scripts);
}

pub fn get_file_path_from_script_struct(script_struct: ScriptStruct) -> String {
    let mut file_path: String = "".to_string();
    for directory in get_config().unwrap().directories {
        if directory.alias == script_struct.alias {
            file_path = format!("{}/{}/{}.sh", directory.path, script_struct.module, script_struct.name);
        }
    }
    return file_path;
}

pub fn get_script_struct_from_fzf_option(fzf_option: String) -> ScriptStruct {
    let fzf_option = if fzf_option.ends_with("\n") {
        fzf_option[..fzf_option.len() - 1].to_string()
    } else {
        fzf_option
    };
    let fzf_option_split: Vec<&str> = fzf_option.split("\t").collect();
    return ScriptStruct {
        alias: fzf_option_split[0].to_string(),
        module: fzf_option_split[1].to_string(),
        name: fzf_option_split[2].to_string(),
    };
}

pub fn get_fzf_option_from_script_struct(script_struct: ScriptStruct) -> String {
    return format!("{}\t{}\t{}", script_struct.alias, script_struct.module, script_struct.name);
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
