use clap::Parser;
use log::LevelFilter;

#[derive(Parser)]
#[clap(about, author, version)]
pub struct Cmd {
    #[clap(short, long, help = "Address to run the server on")]
    pub address: Option<String>,

    #[clap(short, long, help = "Port to run the server on")]
    pub port: Option<u16>,

    #[clap(short, long, help = "Logging level")]
    pub logging_level: Option<LevelFilter>,
}
