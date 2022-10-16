use std::{
    io::{BufRead, BufReader},
    process::Command,
};

use anyhow::{bail, Context, Result};
use log::{debug, log_enabled, trace, warn};

pub async fn run_cmd(cmd: &str, args: &[&str]) -> Result<String> {
    let mut cmd = Command::new(cmd);
    cmd.args(args);

    run_custom_cmd(cmd).await
}

pub async fn run_custom_cmd(cmd: Command) -> Result<String> {
    if log_enabled!(log::Level::Trace) {
        trace!("Going to spawn command runner for: {cmd:?}");
    }

    tokio::spawn(async move { thread_locking_run_cmd(cmd) })
        .await
        .context("Spawned task failed")?
}

pub fn thread_locking_run_cmd(mut cmd: Command) -> Result<String> {
    if log_enabled!(log::Level::Debug) {
        debug!("Going to run command: {cmd:?}");
    }

    let (reader, writer) = os_pipe::pipe().context("Failed to obtain a pipe")?;

    cmd.stdout(
        writer
            .try_clone()
            .context("Failed to clone temporary pipe writer")?,
    );

    cmd.stderr(writer);

    let mut handle = cmd.spawn().context("Failed to spawn the command")?;

    if log_enabled!(log::Level::Debug) {
        debug!("Started command: {cmd:?}");
    }

    let cmd_str = format!("{cmd:?}");

    std::mem::drop(cmd);

    let output_lines = BufReader::new(reader)
        .lines()
        .collect::<std::result::Result<Vec<_>, _>>()
        .context("Failed to get command's output")?;

    let status = handle.wait().context("Failed to wait for command")?;

    if log_enabled!(log::Level::Trace) {
        trace!("Command ended: {cmd_str:?}");
    }

    if status.success() {
        return Ok(output_lines.join("\n"));
    }

    let err = format!(
        "Failed to run command '{cmd_str:?}' (exit code: {}). Output:\n\n{}",
        match status.code() {
            Some(code) => code.to_string(),
            None => "<no code>".to_string(),
        },
        output_lines
            .iter()
            .map(|line| format!(">    {line}"))
            .collect::<Vec<_>>()
            .join("\n"),
    );

    warn!("{err}");
    bail!("{err}")
}
