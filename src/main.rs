#![forbid(unsafe_code)]
#![forbid(unused_must_use)]

mod cmd;
mod docker;
mod server;
mod utils;

use self::{cmd::Cmd, server::StateConfig};
use clap::Parser;
use log::{info, LevelFilter};

#[tokio::main]
async fn main() {
    env_logger::builder()
        .format_target(false)
        .filter_level(LevelFilter::Info)
        .init();

    info!("Application is starting...");

    let cmd = Cmd::parse();

    let config = StateConfig {
        address: cmd.address.unwrap_or_else(|| "127.0.0.1".into()),
        port: cmd.port,
    };

    if let Err(err) = inner_main(config).await {
        eprintln!("Onix failed: {err}");
    }
}

async fn inner_main(config: StateConfig) -> Result<(), String> {
    server::start(config).await
}
