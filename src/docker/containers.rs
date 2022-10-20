use std::{
    collections::{BTreeMap, HashMap},
    fmt::{Display, Formatter},
};

use anyhow::{bail, Context, Result};
use async_graphql::{InputObject, SimpleObject};
use bollard::{
    container::{Config, CreateContainerOptions, ListContainersOptions},
    image::CreateImageOptions,
    models::Mount,
    service::{
        ContainerCreateResponse, ContainerSummary, HostConfig, PortBinding, RestartPolicy,
        RestartPolicyNameEnum,
    },
    Docker,
};
use futures::TryStreamExt;
use log::info;
use serde::{Deserialize, Serialize};

use super::Port;

pub async fn create_container(
    docker: &Docker,
    config: ContainerCreationConfig,
) -> Result<ContainerCreateResponse> {
    info!(
        "==> Pulling image '{}' for container '{}'...",
        config.image, config.name
    );

    docker
        .create_image(
            Some(CreateImageOptions {
                from_image: config.image.as_str(),
                ..Default::default()
            }),
            None,
            None,
        )
        .try_collect::<Vec<_>>()
        .await
        .with_context(|| {
            format!(
                "Failed to pull image '{}' for container '{}'",
                config.image, config.name
            )
        })?;

    #[deny(unused_variables)]
    let ContainerCreationConfig {
        name,
        image,
        env,
        anon_volumes,
        mounts,
        port_bindings,
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

            port_bindings: Some(
                port_bindings
                    .into_iter()
                    .map(|binding| {
                        (
                            binding.container_port.to_docker_port(),
                            Some(vec![PortBinding {
                                host_ip: None,
                                host_port: Some(binding.host_port.to_docker_port()),
                            }]),
                        )
                    })
                    .collect(),
            ),

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

    info!("==> Creating container '{name}'...");

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
    pub port_bindings: Vec<ContainerPortBinding>,
    pub labels: HashMap<String, String>,
    pub restart_policy: ContainerRestartPolicy,
}

#[derive(
    SimpleObject, InputObject, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord,
)]
#[graphql(input_name = "ContainerPortBindingInput")]
pub struct ContainerPortBinding {
    pub host_port: Port,
    pub container_port: Port,
}

impl ContainerPortBinding {
    pub fn collides_with(&self, other: Self) -> bool {
        self.host_port.collides_with(other.host_port)
            || self.container_port.collides_with(other.container_port)
    }

    pub fn find_collision(bindings: &[Self]) -> Option<(Self, Self)> {
        bindings.iter().find_map(|binding| {
            bindings
                .iter()
                .find(|other_binding| binding.collides_with(**other_binding))
                .map(|other_binding| (*binding, *other_binding))
        })
    }
}

impl Display for ContainerPortBinding {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(host) {} <=> {} (container)",
            self.host_port, self.container_port
        )
    }
}

pub struct ContainerMount {
    pub in_host: String,
    pub in_container: String,
    pub readonly: bool,
}
#[derive(Debug)]
pub enum ContainerRestartPolicy {
    #[allow(dead_code)]
    None,

    #[allow(dead_code)]
    UnlessStopped,

    #[allow(dead_code)]
    Always,
}

pub async fn list_containers(docker: &Docker) -> Result<Vec<ExistingContainer>> {
    let containers = docker
        .list_containers::<String>(Some(ListContainersOptions {
            all: true,
            ..Default::default()
        }))
        .await
        .context("Failed to fetch the list of existing Docker containers")?;

    let list = containers
        .into_iter()
        .map(decode_container)
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to analyze the list of existing Docker containers")?;

    Ok(list)
}

fn decode_container(summary: ContainerSummary) -> Result<ExistingContainer> {
    Ok(ExistingContainer {
        docker_container_id: summary.id.context("Missing ID")?,
        names: summary.names.context("Missing names")?,
        labels: summary.labels.context("Missing labels")?,
        status: ExistingContainerStatus::decode(&summary.status.context("Missing status")?)?,
    })
}

pub struct ExistingContainer {
    pub docker_container_id: String,
    pub names: Vec<String>,
    pub labels: HashMap<String, String>,
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
        if input.starts_with("Up ") {
            return Ok(Self::Running);
        }

        match input {
            "Created" => Ok(Self::Created),
            "Restarting" => Ok(Self::Restarting),
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