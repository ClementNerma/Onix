use anyhow::{Context, Result};
use async_graphql::SimpleObject;

use crate::docker::{
    ExistingContainer, ExistingContainerStatus, APP_ID_LABEL, APP_NAME_LABEL, CONTAINER_ID_LABEL,
    CONTAINER_NAME_LABEL, NAME_PREFIX,
};

use super::{AppContainerId, AppId};

#[derive(SimpleObject)]
pub struct ExistingAppContainer {
    pub docker_container_id: String,
    pub docker_container_name: String,
    pub app_id: AppId,
    pub app_name: String,
    pub container_id: AppContainerId,
    pub container_name: String,
    pub status: ExistingContainerStatus,
}

impl ExistingAppContainer {
    pub fn decode(from: ExistingContainer) -> Result<Option<Self>> {
        let ExistingContainer {
            docker_container_id,
            names,
            labels,
            status,
        } = from;

        if names.len() != 1 {
            return Ok(None);
        }

        let name = &names[0];

        let docker_container_name = if let Some(name) = name.strip_prefix('/') {
            name.to_string()
        } else {
            name.to_string()
        };

        if !docker_container_name.starts_with(NAME_PREFIX) {
            return Ok(None);
        }

        let app_id = labels
            .get(APP_ID_LABEL)
            .context("Missing label for application ID")?;

        let app_id = AppId::decode(app_id).context("Failed to parse application ID")?;

        let app_name = labels
            .get(APP_NAME_LABEL)
            .context("Missing label for application name")?
            .clone();

        let container_id = labels
            .get(CONTAINER_ID_LABEL)
            .context("Missing label for container ID")?;

        let container_id =
            AppContainerId::decode(container_id).context("Failed to parse container ID")?;

        let container_name = labels
            .get(CONTAINER_NAME_LABEL)
            .context("Missing label for container name")?
            .clone();

        Ok(Some(Self {
            docker_container_id,
            docker_container_name,
            app_id,
            app_name,
            container_id,
            container_name,
            status,
        }))
    }
}
