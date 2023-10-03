use std::io::{self, Write};
use std::process::{Command, exit, Stdio};
use crate::config_service::{Config};
use crate::files_service::{get_file_path_from_script_struct, get_fzf_option_from_script_struct, get_script_struct_from_fzf_option, get_scripts, ScriptStruct};

pub fn launch_script(config: Config) -> io::Result<()> {
    let scripts: Vec<ScriptStruct>;
    match get_scripts(config) {
        Ok(script) => {
            scripts = script;
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    }
    let scripts_parsed_string: String = scripts.iter().map(|script: &ScriptStruct| {
        get_fzf_option_from_script_struct(script.clone())
    }).collect::<Vec<String>>().join("\n");
    let scripts_parsed = scripts_parsed_string.as_bytes();

    let mut cmd = Command::new("fzf")
        .arg("--ansi")
        .arg("--prompt=Select an script >")
        .arg("--preview-window=right:50%:wrap")
        .arg("--preview=poldot --doc_parse --file_path {}")
        .arg("--layout=reverse")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut fzf_stdin = cmd.stdin.take().unwrap();

    fzf_stdin.write_all(scripts_parsed)?;
    fzf_stdin.flush()?;
    drop(fzf_stdin);

    let output = cmd.wait_with_output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);

    println!("Resultado: {}", stdout);

    if stdout.is_empty() {
        exit(1);
    }

    let script_struct: ScriptStruct = get_script_struct_from_fzf_option(stdout.to_string());
    let file_path: String = get_file_path_from_script_struct(script_struct);
    println!("File path: {}", file_path);

    let mut execution = Command::new("bash")
        .arg(file_path)
        .stdout(Stdio::piped())
        .spawn()?;
    execution.wait()?;

    Ok(())
}
