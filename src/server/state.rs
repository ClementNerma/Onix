use std::collections::BTreeMap;

use bollard::Docker;

use crate::apps::App;

pub struct State {
    pub port: u16,
    pub address: String,
    pub docker: Docker,
    pub apps: BTreeMap<String, App>,
}

impl State {
    pub fn new(
        StateConfig {
            port,
            address,
            docker,
        }: StateConfig,
    ) -> Self {
        State {
            port,
            address,
            docker,
            apps: BTreeMap::new(),
        }
    }
}

pub struct StateConfig {
    pub port: u16,
    pub address: String,
    pub docker: Docker,
}
