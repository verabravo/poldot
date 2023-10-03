use std::io;
use clap::{Arg, ArgAction, ArgMatches, Command};
use crate::config_service::{Config, get_config};

mod doc_parse;
mod app_launcher;
mod config_service;
mod files_service;

fn cli() -> Command {
    Command::new("poldot")
        .about("script launcher or info printer")
        .version("1.0")
        .arg(Arg::new("doc_parse")
            .long("doc_parse")
            .action(ArgAction::SetTrue)
            .default_value("false")
            .required(false))
        .arg(Arg::new("file_path")
            .long("file_path")
            .required_if_eq("doc_parse", "true"))
}

fn main() -> io::Result<()> {
    let config: Config;
    match get_config() {
        Ok(current_config) => {
            config = current_config;
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            return Ok(());
        }
    }
    let matches: ArgMatches = cli().get_matches();
    let doc_parse: bool = matches.get_flag("doc_parse");
    if true == doc_parse {
        let file_path: String = matches.get_one::<String>("file_path").unwrap().to_string();
        doc_parse::print_documentation(file_path, config);
    } else {
        return app_launcher::launch_script(config);
    }

    Ok(())
}
