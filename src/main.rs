use std::io;
use clap::{Arg, ArgAction, ArgMatches, Command};

mod doc_parse;
mod app_launcher;
mod get_config;

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
    let matches: ArgMatches = cli().get_matches();
    let doc_parse: bool = matches.get_flag("doc_parse");
    if true == doc_parse {
        let file_path: String = matches.get_one::<String>("file_path").unwrap().to_string();
        doc_parse::print_documentation(file_path);
    } else {
        return app_launcher::launch_script();
    }

    Ok(())
}
