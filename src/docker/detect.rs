use anyhow::Result;

use crate::utils::shell::run_cmd;

static DOCKER_CMD: &str = "docker";
static DOCKER_COMPOSE_CMD: &str = "docker compose";

pub fn docker_version() -> Result<String> {
    run_cmd(DOCKER_CMD, &["version"])
}

pub fn docker_compose_version() -> Result<String> {
    run_cmd(DOCKER_COMPOSE_CMD, &["version"])
}
