use anyhow::{Context, Result};
use bollard::Docker;

pub async fn stop_container(docker: &Docker, name: &str) -> Result<()> {
    docker
        .stop_container(name, None)
        .await
        .with_context(|| format!("Failed to stop container '{name}'"))?;

    Ok(())
}
