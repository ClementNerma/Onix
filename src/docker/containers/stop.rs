use anyhow::Result;
use bollard::Docker;

pub async fn stop_container(docker: &Docker, docker_container_id: &str) -> Result<()> {
    todo!()
}
