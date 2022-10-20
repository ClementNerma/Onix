use std::sync::Arc;

use bollard::Docker;
use tokio::sync::{Mutex, MutexGuard};

use crate::{
    apps::{AppId, AppRunner, AppRunnerConfig, AppRunnerEnvironment},
    data::UserData,
    utils::graphql::Result,
};

pub struct WrappedState(Arc<Mutex<State>>);

impl WrappedState {
    pub fn new(config: StateConfig) -> Self {
        Self(Arc::new(Mutex::new(State::new(config))))
    }

    pub async fn lock(&self) -> MutexGuard<State> {
        self.0.lock().await
    }
}

impl Clone for WrappedState {
    fn clone(&self) -> Self {
        Self(Arc::clone(&self.0))
    }
}

pub struct State {
    pub port: u16,
    pub address: String,
    pub docker: Docker,
    pub user_data: UserData,
    pub runner_env: AppRunnerEnvironment,
}

impl State {
    fn new(
        #[deny(unused_variables)] StateConfig {
            port,
            address,
            docker,
            user_data,
            runner_config,
        }: StateConfig,
    ) -> State {
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

pub async fn get_runner_for(state: &State, id: AppId) -> Result<AppRunner> {
    let app = state
        .user_data
        .apps
        .iter()
        .find(|app| app.id == id)
        .ok_or("Provided application ID was not found")?;

    Ok(AppRunner::new(&state.docker, &state.runner_env, app))
}
