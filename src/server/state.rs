pub struct State {
    pub port: u16,
    pub address: String,
}

impl State {
    pub fn new(StateConfig { port, address }: StateConfig) -> Self {
        State { port, address }
    }
}

pub struct StateConfig {
    pub port: u16,
    pub address: String,
}
