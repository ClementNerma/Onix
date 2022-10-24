#![forbid(unsafe_code)]
#![forbid(unused_must_use)]

use onix_server::stores::StoreContent;

use std::{fs, path::PathBuf};

use anyhow::{bail, Context, Result};
use clap::Parser;
use log::{error, info, LevelFilter};

#[derive(Parser)]
#[clap(about, version, author, about = "Create a packed store from JSON")]
struct Cmd {
    #[clap(help = "Path to the JSON file to pack")]
    input_file: PathBuf,

    #[clap(help = "Path the output packed store file")]
    output_file: PathBuf,

    #[clap(short, long, help = "Logging level")]
    pub logging_level: Option<LevelFilter>,
}

fn main() {
    let cmd = Cmd::parse();

    env_logger::builder()
        .filter_level(cmd.logging_level.unwrap_or(LevelFilter::Info))
        .init();

    info!("Running store packer...");

    if let Err(err) = inner_main(cmd) {
        error!("Store packer failed: {err:?}");
    }
}

fn inner_main(cmd: Cmd) -> Result<()> {
    #[deny(unused_variables)]
    let Cmd {
        input_file,
        output_file,

        logging_level: _,
    } = cmd;

    if !input_file.is_file() {
        bail!("Input file does not exist or is not a file.");
    }

    if output_file.exists() {
        bail!("Output path already exists");
    }

    let input = fs::read_to_string(&input_file).context("Failed to read input JSON file")?;

    info!("> Parsing the input file...");

    let store: StoreContent =
        serde_json::from_str(&input).context("Failed to deserialize JSON file")?;

    info!("> Compressing...");

    let compressed = store.compress().context("Failed to compress the store")?;

    info!("> Writing to output file...");

    fs::write(&output_file, &compressed)
        .context("Failed to write compressed store to the output file")?;

    info!("Successfully packed the store!");

    Ok(())
}
