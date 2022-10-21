use std::collections::HashMap;

use anyhow::{Context, Result};
use bollard::{
    image::{CreateImageOptions, ListImagesOptions},
    service::ImageSummary,
    Docker,
};
use futures::TryStreamExt;

pub async fn find_images_by_reference(
    docker: &Docker,
    reference: &str,
) -> Result<Vec<ImageSummary>> {
    docker
        .list_images(Some(ListImagesOptions {
            filters: HashMap::from([("reference", vec![reference])]),
            ..Default::default()
        }))
        .await
        .context("Failed to obtain the list of local Docker images")
}

pub async fn has_image_locally(docker: &Docker, image: &str) -> Result<bool> {
    let images = find_images_by_reference(docker, image).await?;

    Ok(!images.is_empty())
}

pub async fn pull_image(docker: &Docker, image: &str) -> Result<()> {
    docker
        .create_image(
            Some(CreateImageOptions {
                from_image: image,
                ..Default::default()
            }),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .with_context(|| format!("Failed to pull image '{image}'"))?;

    Ok(())
}
