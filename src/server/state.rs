use bollard::Docker;

pub struct State {
    pub port: u16,
    pub address: String,
    pub docker: Docker,
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
        }
    }
}

pub struct StateConfig {
    pub port: u16,
    pub address: String,
    pub docker: Docker,
}
