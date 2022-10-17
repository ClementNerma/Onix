use anyhow::Result;
use bollard::Docker;

pub async fn docker_version(docker: &Docker) -> Result<Option<String>> {
    let version = docker.version().await?;

    Ok(version.version)
}
