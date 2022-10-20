use std::sync::Arc;

use bollard::Docker;
use tokio::sync::Mutex;

use crate::{
    apps::{AppRunner, AppRunnerConfig, AppRunnerEnvironment},
    data::UserData,
    utils::graphql::Result,
};

pub type WrappedState = Arc<Mutex<State>>;

pub struct State {
    pub port: u16,
    pub address: String,
    pub docker: Docker,
    pub user_data: UserData,
    pub runner_env: AppRunnerEnvironment,
}

impl State {
    pub fn new(
        #[deny(unused_variables)] StateConfig {
            port,
            address,
            docker,
            user_data,
            runner_config,
        }: StateConfig,
    ) -> Self {
        State {
            port,
            address,
            docker,
            user_data: user_data.unwrap_or_default(),
            runner_env: AppRunnerEnvironment::new(runner_config),
        }
    }
}

pub struct StateConfig {
    pub port: u16,
    pub address: String,
    pub docker: Docker,
    pub user_data: Option<UserData>,
    pub runner_config: AppRunnerConfig,
}

pub async fn get_runner_for(state: &State, app_id: u64) -> Result<AppRunner> {
    let app = state
        .user_data
        .apps
        .iter()
        .find(|app| app.id == app_id)
        .ok_or("Provided application ID was not found")?;

    Ok(AppRunner::new(&state.docker, &state.runner_env, app))
}
