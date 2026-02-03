use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Serialize, Debug)]
pub struct CmdResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub success: bool,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum ExecMode {
    Exec {
        program: String,
        #[serde(default)]
        args: Vec<String>,
    },
    Shell {
        cmd: String,
    },
}

// Running Shell Commands
/// Executes a Shell Command and Returns (Stdout, Stderr)
pub fn execute(mode: ExecMode) -> std::io::Result<CmdResult> {
    let output = match mode {
        ExecMode::Exec { program, args } => Command::new(program).args(args).output()?,
        ExecMode::Shell { cmd } => Command::new("bash").arg("-lc").arg(cmd).output()?,
    };

    Ok(CmdResult {
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        exit_code: output.status.code().unwrap_or(-1),
        success: output.status.success(),
    })
}

/// If you specifically want a JSON string for IPC:
pub fn execute_json(mode: ExecMode) -> std::io::Result<String> {
    let res = execute(mode)?;
    Ok(serde_json::to_string(&res).unwrap())
}
