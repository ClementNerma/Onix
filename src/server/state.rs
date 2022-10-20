use std::sync::Arc;

use anyhow::Result;
use async_graphql::Context;
use bollard::Docker;
use tokio::sync::{Mutex, MutexGuard};

use crate::{
    apps::{AppId, AppRunner, AppRunnerConfig, AppRunnerEnvironment},
    data::UserData,
};

use super::user_data::{UserDataSaver, UserDataSavingState, WritableUserData};

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
    pub runner_env: AppRunnerEnvironment,
    pub user_data_saver: UserDataSaver,
    pub user_data_saving_state: UserDataSavingState,

    user_data: UserData,
}

impl State {
    fn new(
        #[deny(unused_variables)] StateConfig {
            port,
            address,
            docker,
            user_data,
            user_data_saver,
            runner_config,
        }: StateConfig,
    ) -> State {
        State {
            port,
            address,
            docker,
            runner_env: AppRunnerEnvironment::new(runner_config),
            user_data_saver,
            user_data_saving_state: UserDataSavingState::Unchanged,

            user_data: user_data.unwrap_or_default(),
        }
    }

    pub fn user_data(&self) -> &UserData {
        &self.user_data
    }

    pub fn user_data_mut(&mut self) -> WritableUserData {
        WritableUserData::new(&mut self.user_data, &mut self.user_data_saving_state)
    }
}

pub struct StateConfig {
    pub port: u16,
    pub address: String,
    pub docker: Docker,
    pub user_data: Option<UserData>,
    pub user_data_saver: UserDataSaver,
    pub runner_config: AppRunnerConfig,
}

pub async fn get_runner_for(state: &State, id: AppId) -> Result<AppRunner, String> {
    let app = state
        .user_data
        .apps
        .iter()
        .find(|app| app.id == id)
        .ok_or("Provided application ID was not found")?;

    Ok(AppRunner::new(&state.docker, &state.runner_env, app))
}

pub async fn get_state<'a>(context: &'a Context<'_>) -> MutexGuard<'a, State> {
    context
        .data::<WrappedState>()
        .expect("Assertion error: GraphQL context does not have the expected type")
        .lock()
        .await
}
