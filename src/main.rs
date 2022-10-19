#![forbid(unsafe_code)]
#![forbid(unused_must_use)]

mod apps;
mod cmd;
mod data;
mod docker;
mod server;
mod utils;

use self::{cmd::Cmd, server::StateConfig};
use anyhow::{Context, Result};
use bollard::Docker;
use clap::Parser;
use log::{info, LevelFilter};

#[tokio::main]
async fn main() {
    let cmd = Cmd::parse();

    env_logger::builder()
        .format_module_path(false)
        .filter_level(cmd.logging_level.unwrap_or(LevelFilter::Info))
        .init();

    info!("Application is starting...");

    if let Err(err) = inner_main(cmd).await {
        eprintln!("Onix failed: {err:?}");
    }
}

async fn inner_main(cmd: Cmd) -> Result<()> {
    let docker = Docker::connect_with_local_defaults().context("Failed to connect to Docker")?;

    let config = StateConfig {
        address: cmd.address.unwrap_or_else(|| "127.0.0.1".into()),
        port: cmd.port,
        docker,

        // TODO: store user data somewhere
        user_data: None,
    };

    server::start(config).await
}
