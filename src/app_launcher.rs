use std::io::{self, Write};
use std::process::{Command, exit, Stdio};
use crate::get_config::{get_scripts, ScriptStruct};

pub fn launch_script() -> io::Result<()> {
    let scripts: Vec<ScriptStruct>;
    match get_scripts() {
        Ok(script) => {
            scripts = script;
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            exit(1);
        }
    }
    let scripts_parsed_string: String = scripts.iter().map(|script: &ScriptStruct| {
        format!("{} - {} - {}", script.alias, script.module, script.name)
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

    Ok(())
}
