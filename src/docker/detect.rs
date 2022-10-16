use crate::utils::shell::{run_cmd, CmdError};

static DOCKER_CMD: &str = "docker";
static DOCKER_COMPOSE_CMD: &str = "docker compose";

pub fn docker_version() -> Result<String, CmdError> {
    run_cmd(DOCKER_CMD, &["version"])
}
