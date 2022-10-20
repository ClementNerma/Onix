use std::collections::{BTreeMap, HashMap};

use anyhow::{Context, Result};
use bollard::{
    container::{Config, CreateContainerOptions},
    models::Mount,
    service::{ContainerCreateResponse, HostConfig, RestartPolicy, RestartPolicyNameEnum},
    Docker,
};

pub async fn create_container(
    docker: &Docker,
    config: ContainerCreationConfig,
) -> Result<ContainerCreateResponse> {
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
