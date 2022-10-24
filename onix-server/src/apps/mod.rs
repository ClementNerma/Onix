mod app;
mod containers;
mod env;
mod existing_containers;
mod runner;
mod templates;

pub use app::{App, AppId};
pub use containers::{AppContainer, AppContainerId};
pub use env::{AppRunnerConfig, AppRunnerEnvironment};
pub use existing_containers::ExistingAppContainer;
pub use runner::{AppRunner, AppRunningStatus};
pub use templates::*;

use once_cell::sync::Lazy;
use pomsky_macro::pomsky;
use regex::Regex;

pub static NAME_VALIDATOR: Lazy<Regex> = Lazy::new(|| {
    Regex::new(pomsky!(
        Start ['a'-'z' 'A'-'Z' '0'-'9' '-' '_']+ End
    ))
    .unwrap()
});
