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

/// The application's state
pub struct State {
    /// Port the server is running on
    pub port: u16,

    /// Address the server is running on
    pub address: String,

    /// Docker API client
    pub docker: Docker,

    /// Runner environment
    pub runner_env: AppRunnerEnvironment,

    /// Function used to save user data when modified
    pub user_data_saver: UserDataSaver,

    /// Modification state of the user data
    pub user_data_saving_state: UserDataSavingState,

    /// The user data
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

/// Configuration object used to generate a state
pub struct StateConfig {
    pub port: u16,
    pub address: String,
    pub docker: Docker,
    pub user_data: Option<UserData>,
    pub user_data_saver: UserDataSaver,
    pub runner_config: AppRunnerConfig,
}

/// Wrapper for the server's state, used to synchronize it across multiple threads
#[derive(Clone)]
pub struct WrappedState(Arc<Mutex<State>>);

impl WrappedState {
    pub fn new(config: StateConfig) -> Self {
        Self(Arc::new(Mutex::new(State::new(config))))
    }

    /// Get the inner state, locking it across all threads
    pub async fn lock(&self) -> MutexGuard<State> {
        self.0.lock().await
    }
}

/// Get a readable and writable state from a GraphQL context
pub async fn get_state<'a>(context: &'a Context<'_>) -> MutexGuard<'a, State> {
    context
        .data::<WrappedState>()
        .expect("Assertion error: GraphQL context does not have the expected type")
        .lock()
        .await
}

/// Generate a runner for a specific application
pub async fn get_runner_for(state: &State, id: AppId) -> Result<AppRunner, String> {
    let app = state
        .user_data
        .apps
        .iter()
        .find(|app| app.id == id)
        .ok_or("Provided application ID was not found")?;

    Ok(AppRunner::new(&state.docker, &state.runner_env, app))
}
