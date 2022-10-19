use std::sync::Arc;

use bollard::Docker;
use tokio::sync::Mutex;

use crate::data::UserData;

pub type WrappedState = Arc<Mutex<State>>;

pub struct State {
    pub port: u16,
    pub address: String,
    pub docker: Docker,
    pub user_data: UserData,
}

impl State {
    pub fn new(
        StateConfig {
            port,
            address,
            docker,
            user_data,
        }: StateConfig,
    ) -> Self {
        State {
            port,
            address,
            docker,
            user_data: user_data.unwrap_or_default(),
        }
    }
}

pub struct StateConfig {
    pub port: u16,
    pub address: String,
    pub docker: Docker,
    pub user_data: Option<UserData>,
}
