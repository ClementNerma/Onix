use std::process::{Command, Output};

use anyhow::{bail, Context, Result};

pub async fn run_cmd(cmd: &str, args: &[&str]) -> Result<String> {
    let mut cmd = Command::new(cmd);
    cmd.args(args);

    run_custom_cmd(cmd).await
}

pub async fn run_custom_cmd(mut cmd: Command) -> Result<String> {
    tokio::spawn(async move {
        let output = cmd
            .output()
            .with_context(|| format!("Failed to run command '{cmd:?}'"))?;

        ensure_cmd_success(&cmd, &output)?;

        Ok(String::from_utf8_lossy(&output.stdout).into_owned())
    })
    .await
    .context("Spawned task failed")?
}

pub fn ensure_cmd_success(cmd: &Command, output: &Output) -> Result<()> {
    if output.status.success() {
        Ok(())
    } else {
        bail!(
            "Failed to run command '{cmd:?}' (exit code: {}):\n\nSTDOUT:\n\n{}\n\nSTDERR:\n\n{}",
            match output.status.code() {
                Some(code) => code.to_string(),
                None => "<no code>".to_string(),
            },
            String::from_utf8_lossy(&output.stdout)
                .lines()
                .map(|line| format!("    {line}"))
                .collect::<Vec<_>>()
                .join("\n"),
            String::from_utf8_lossy(&output.stderr)
                .lines()
                .map(|line| format!("    {line}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
