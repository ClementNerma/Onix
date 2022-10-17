use anyhow::{Context, Result};
use bollard::{
    container::{Config, CreateContainerOptions},
    service::ContainerCreateResponse,
    Docker,
};

pub async fn docker_version(docker: &Docker) -> Result<Option<String>> {
    let version = docker.version().await?;

    Ok(version.version)
}

pub async fn create_container(
    docker: &Docker,
    container_name: String,
    config: Config<String>,
) -> Result<ContainerCreateResponse> {
    docker
        .create_container(
            Some(CreateContainerOptions {
                name: container_name,
            }),
            config,
        )
        .await
        .context("Failed to create Docker container")
}
