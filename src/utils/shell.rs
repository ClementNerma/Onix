use paste::paste;
use std::process::{Command, Output};

use crate::graphqlify;

graphqlify!(
    pub enum CmdError {
        FailedToRun {
            message: String,
            command: String,
        },
        CommandFailed {
            exit_code: Option<i32>,
            command: String,
            stdout: String,
            stderr: String,
        },
    }
);

pub fn run_cmd(cmd: &str, args: &[&str]) -> Result<String, CmdError> {
    run_custom_cmd(Command::new(cmd).args(args))
}

pub fn run_custom_cmd(cmd: &mut Command) -> Result<String, CmdError> {
    let output = cmd.output().map_err(|err| CmdError::FailedToRun {
        command: format!("{cmd:?}"),
        message: format!("{err}"),
    })?;

    ensure_cmd_success(&cmd, &output)?;

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn ensure_cmd_success(cmd: &Command, output: &Output) -> Result<(), CmdError> {
    if output.status.success() {
        Ok(())
    } else {
        Err(CmdError::CommandFailed {
            command: format!("{cmd:?}"),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}
