#![forbid(unsafe_code)]
#![forbid(unused_must_use)]

mod apps;
mod cmd;
mod data;
mod docker;
mod saving;
mod server;
mod utils;

use self::cmd::Cmd;
use self::utils::time::get_now;

use anyhow::{Context, Result};
use apps::AppRunnerConfig;
use bollard::Docker;
use clap::Parser;
use log::{info, LevelFilter};
use saving::{save_user_data, try_load_user_data};
use server::StateConfig;

#[tokio::main]
async fn main() {
    // Trigger offest fetching
    get_now();

    let cmd = Cmd::parse();

    env_logger::builder()
        .filter_level(cmd.logging_level.unwrap_or(LevelFilter::Info))
        .init();

    info!("Application is starting...");

    if let Err(err) = inner_main(cmd).await {
        eprintln!("Onix failed: {err:?}");
    }
}

async fn inner_main(cmd: Cmd) -> Result<()> {
    let docker = Docker::connect_with_socket_defaults().context("Failed to connect to Docker")?;

    let data_dir = dirs::data_local_dir()
        .context("Failed to obtain path to local data directory")?
        .join("onix");

    let config = StateConfig {
        address: cmd.address.unwrap_or_else(|| "127.0.0.1".into()),
        port: cmd.port.unwrap_or(5871),
        docker,

        // TODO: configurable directories
        runner_config: AppRunnerConfig {
            data_dir: data_dir.clone(),
        },

        // TODO: load user data from disk
        user_data: try_load_user_data(&data_dir)?,

        user_data_saver: Box::new(move |user_data| save_user_data(&data_dir, user_data.clone())),
    };

    server::start(config).await
}
