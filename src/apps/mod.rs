mod app;
mod containers;
mod env;
mod runner;
mod volumes;

pub use app::{App, AppCreationInput};
pub use env::{AppRunnerConfig, AppRunnerEnvironment};
pub use runner::{AppRunner, AppRunningStatus};

use once_cell::sync::Lazy;
use pomsky_macro::pomsky;
use regex::Regex;

pub static NAME_VALIDATOR: Lazy<Regex> = Lazy::new(|| {
    Regex::new(pomsky!(
        Start ['a'-'z' 'A'-'Z' '0'-'9' '-' '_']+ End
    ))
    .unwrap()
});
