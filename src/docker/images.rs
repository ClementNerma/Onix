use anyhow::{Context, Result};
use bollard::{service::ImageSummary, Docker};

pub async fn list_images(docker: &Docker) -> Result<Vec<ImageSummary>> {
    docker
        .list_images::<String>(None)
        .await
        .context("Failed to list existing Docker images")
}
