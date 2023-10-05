use std::fs::File;
use std::io;
use std::io::{Read, Write};
use crate::config_service::Config;
use crate::files_service::{get_file_path_from_script_struct, get_script_struct_from_fzf_option, is_a_fzf_option};

pub fn print_documentation(mut file_path: String, config: Config) -> io::Result<()> {
    if true == is_a_fzf_option(file_path.clone()) {
        let script_struct = get_script_struct_from_fzf_option(file_path);
        file_path = get_file_path_from_script_struct(script_struct, config.clone());
    }
    let mut file;

    match File::open(file_path.clone()) {
        Ok(file_opened) => {
            file = file_opened;
        }
        Err(_) => {
            eprintln!("File not found: {}", file_path);
            return Ok(());
        }
    }

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    io::stdout().write_all(&buffer)?;

    Ok(())
}