use crate::{
    apps::{AppContainerId, AppId},
    docker::{APP_ID_LABEL, APP_NAME_LABEL, CONTAINER_ID_LABEL, CONTAINER_NAME_LABEL, NAME_PREFIX},
};

use std::collections::{BTreeMap, HashMap};

use anyhow::{bail, Context, Result};
use bollard::{
    container::{Config, CreateContainerOptions},
    image::CreateImageOptions,
    models::Mount,
    service::{
        ContainerCreateResponse, ContainerSummary, HostConfig, RestartPolicy, RestartPolicyNameEnum,
    },
    Docker,
};
use futures::TryStreamExt;

pub async fn create_container(
    docker: &Docker,
    config: ContainerCreationConfig,
) -> Result<ContainerCreateResponse> {
    // ===== REQUIRED UNTIL CORRECT OPTIONS ARE ADDED TO BOLLARD ===== //

    docker
        .create_image(
            Some(CreateImageOptions {
                from_image: config.name.clone(),
                ..Default::default()
            }),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await?;

    // =============================================================== //

    #[deny(unused_variables)]
    let ContainerCreationConfig {
        name,
        image,
        env,
        anon_volumes,
        mounts,
        labels,
        restart_policy,
    } = config;

    let config = Config {
        image: Some(image.clone()),

        labels: Some(labels),

        env: Some(
            env.iter()
                .map(|(name, value)| format!("{name}={value}"))
                .collect(),
        ),

        volumes: Some(
            anon_volumes
                .into_iter()
                .map(|key| {
                    let mut empty = HashMap::new();
                    empty.insert((), ());
                    (key.clone(), empty)
                })
                .collect(),
        ),

        host_config: Some(HostConfig {
            restart_policy: Some(RestartPolicy {
                name: Some(match restart_policy {
                    ContainerRestartPolicy::None => RestartPolicyNameEnum::NO,
                    ContainerRestartPolicy::UnlessStopped => RestartPolicyNameEnum::UNLESS_STOPPED,
                    ContainerRestartPolicy::Always => RestartPolicyNameEnum::ALWAYS,
                }),
                maximum_retry_count: None,
            }),

            mounts: Some(
                mounts
                    .into_iter()
                    .map(
                        |ContainerMount {
                             in_host,
                             in_container,
                             readonly,
                         }| Mount {
                            source: Some(in_host),
                            target: Some(in_container),
                            read_only: Some(readonly),
                            ..Default::default()
                        },
                    )
                    .collect(),
            ),

            ..Default::default()
        }),

        ..Default::default()
    };

    docker
        .create_container(Some(CreateContainerOptions { name }), config)
        .await
        .context("Failed to create Docker container")
}

pub struct ContainerCreationConfig {
    pub name: String,
    pub image: String,
    pub env: BTreeMap<String, String>,
    pub anon_volumes: Vec<String>,
    pub mounts: Vec<ContainerMount>,
    pub labels: HashMap<String, String>,
    pub restart_policy: ContainerRestartPolicy,
}

pub struct ContainerMount {
    pub in_host: String,
    pub in_container: String,
    pub readonly: bool,
}

pub enum ContainerRestartPolicy {
    None,
    UnlessStopped,
    Always,
}

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
        app_id: AppId(app_id),
        app_name,
        container_id: AppContainerId(container_id),
        container_name,
        status,
    }))
}

pub struct ExistingContainer {
    pub docker_container_id: String,
    pub app_id: AppId,
    pub app_name: String,
    pub container_id: AppContainerId,
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
}

pub async fn start_container(docker: &Docker, container_name: &str) -> Result<()> {
    docker
        .start_container::<String>(container_name, None)
        .await
        .with_context(|| format!("Failed to start container '{container_name}'"))
}

pub async fn stop_container(docker: &Docker, name: &str) -> Result<()> {
    docker
        .stop_container(name, None)
        .await
        .with_context(|| format!("Failed to stop container '{name}'"))?;

    Ok(())
}
