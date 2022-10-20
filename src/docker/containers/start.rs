use anyhow::{Context, Result};
use bollard::Docker;

pub async fn start_container(docker: &Docker, container_name: &str) -> Result<()> {
    docker
        .start_container::<String>(container_name, None)
        .await
        .with_context(|| format!("Failed to start container '{container_name}'"))
}
