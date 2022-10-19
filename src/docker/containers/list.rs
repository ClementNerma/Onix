use anyhow::{bail, Context, Result};
use bollard::{service::ContainerSummary, Docker};

use crate::docker::{
    APP_ID_LABEL, APP_NAME_LABEL, CONTAINER_ID_LABEL, CONTAINER_NAME_LABEL, NAME_PREFIX,
};

pub async fn list_containers(docker: &Docker) -> Result<Vec<ExistingContainer>> {
    let containers = docker
        .list_containers::<String>(None)
        .await
        .context("Failed to fetch the list of existing Docker containers")?;

    let list = containers
        .into_iter()
        .filter_map(|summary| match decode_container(summary) {
            Ok(Some(container)) => Some(Ok(container)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        })
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to analyze the list of existing Docker containers")?;

    Ok(list)
}

fn decode_container(summary: ContainerSummary) -> Result<Option<ExistingContainer>> {
    let names = match summary.names {
        Some(names) => names,
        None => return Ok(None),
    };

    if names.len() != 1 {
        return Ok(None);
    }

    if !names[0].starts_with(NAME_PREFIX) {
        return Ok(None);
    }

    let labels = summary.labels.context("Missing container labels")?;

    let app_id = labels
        .get(APP_ID_LABEL)
        .context("Missing label for application ID")?
        .parse()
        .context("Failed to parse application ID")?;

    let app_name = labels
        .get(APP_NAME_LABEL)
        .context("Missing label for application name")?
        .clone();

    let container_id = labels
        .get(CONTAINER_ID_LABEL)
        .context("Missing label for container ID")?
        .parse()
        .context("Failed to parse container ID")?;

    let container_name = labels
        .get(CONTAINER_NAME_LABEL)
        .context("Missing label for container name")?
        .clone();

    let status = ExistingContainerStatus::decode(&summary.status.context("Missing status")?)?;

    Ok(Some(ExistingContainer {
        docker_container_id: summary.id.context("Missing ID")?,
        app_id,
        app_name,
        container_id,
        container_name,
        status,
    }))
}

pub struct ExistingContainer {
    pub docker_container_id: String,
    pub app_id: u64,
    pub app_name: String,
    pub container_id: u64,
    pub container_name: String,
    pub status: ExistingContainerStatus,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum ExistingContainerStatus {
    Created,
    Restarting,
    Running,
    Removing,
    Paused,
    Exited,
    Dead,
}

impl ExistingContainerStatus {
    pub fn decode(input: &str) -> Result<Self> {
        match input {
            "Created" => Ok(Self::Created),
            "Restarting" => Ok(Self::Restarting),
            "Running" => Ok(Self::Running),
            "Removing" => Ok(Self::Removing),
            "Paused" => Ok(Self::Paused),
            "Exited" => Ok(Self::Exited),
            "Dead" => Ok(Self::Dead),
            _ => bail!("Invalid container status: {}", input),
        }
    }

    pub fn running_like(&self) -> bool {
        match self {
            Self::Created | Self::Restarting | Self::Running | Self::Paused => true,
            Self::Removing | Self::Exited | Self::Dead => false,
        }
    }
}
