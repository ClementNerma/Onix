use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    sync::Arc,
};

use anyhow::{bail, Context, Result};
use async_graphql::Enum;
use bollard::Docker;
use futures::future::try_join_all;
use log::info;

use crate::{
    apps::volumes::AppVolume,
    docker::{
        self, ContainerCreationConfig, ContainerMount, ContainerRestartPolicy,
        ExistingContainerStatus, APP_ID_LABEL, APP_NAME_LABEL, CONTAINER_ID_LABEL,
        CONTAINER_NAME_LABEL,
    },
};

use super::{
    app::App, containers::AppContainer, env::AppRunnerEnvironment,
    existing_containers::ExistingAppContainer,
};

pub struct AppRunner<'a, 'b, 'c> {
    docker: &'a Docker,
    env: &'b AppRunnerEnvironment,
    app: &'c App,
}

impl<'a, 'b, 'c> AppRunner<'a, 'b, 'c> {
    pub fn new(docker: &'a Docker, env: &'b AppRunnerEnvironment, app: &'c App) -> Self {
        Self { docker, env, app }
    }

    async fn list_existing_containers(&self) -> Result<Vec<ExistingAppContainer>> {
        let containers = docker::list_containers(&self.docker)
            .await
            .context("Failed to obtain the list of existing Docker containers")?;

        containers
            .into_iter()
            .filter_map(|container| self.app.decode_container(container).transpose())
            .collect::<Result<Vec<_>, _>>()
    }

    pub async fn status(&self) -> Result<AppRunningStatus> {
        let container_ids = self
            .app
            .containers
            .iter()
            .map(|container| container.id)
            .collect::<HashSet<_>>();

        let existing = self.list_existing_containers().await?;

        let existing = existing
            .into_iter()
            .filter(|existing| {
                existing.app_id == self.app.id && container_ids.contains(&existing.container_id)
            })
            .collect::<Vec<_>>();

        let existing_ids = existing
            .iter()
            .map(|existing| existing.container_id)
            .collect::<HashSet<_>>();

        let created_count = self
            .app
            .containers
            .iter()
            .filter(|container| existing_ids.contains(&container.id))
            .count();

        if created_count == 0 {
            return Ok(AppRunningStatus::NotCreated);
        }

        if created_count < self.app.containers.len() {
            return Ok(AppRunningStatus::PartiallyCreated);
        }

        let statuses = existing
            .iter()
            .map(|existing| existing.status)
            .collect::<Vec<_>>();

        if statuses.contains(&ExistingContainerStatus::Dead) {
            return Ok(AppRunningStatus::Zombie);
        }

        if statuses.contains(&ExistingContainerStatus::Paused)
            || statuses.contains(&ExistingContainerStatus::Removing)
            || statuses.contains(&ExistingContainerStatus::Restarting)
        {
            return Ok(AppRunningStatus::Intermediary);
        }

        if statuses.iter().all(|status| {
            *status == ExistingContainerStatus::Created
                || *status == ExistingContainerStatus::Exited
        }) {
            return Ok(AppRunningStatus::Stopped);
        }

        if statuses.contains(&ExistingContainerStatus::Exited) {
            return Ok(AppRunningStatus::PartiallyRunning);
        }

        assert!(
            statuses
                .iter()
                .all(|status| *status == ExistingContainerStatus::Running),
            "Assertion error: invalid predicates on existing container status"
        );

        Ok(AppRunningStatus::FullyRunning)
    }

    pub async fn create_containers(&self) -> Result<()> {
        info!(
            "Creating containers for application '{}' [{}]...",
            self.app.name, self.app.id
        );

        match self.status().await? {
            AppRunningStatus::NotCreated => {}
            AppRunningStatus::PartiallyCreated => {
                bail!("Some of the application's containers is/are already created")
            }
            AppRunningStatus::Zombie => bail!("At least one container is in zombie mode"),
            AppRunningStatus::Intermediary => {
                bail!("At least one container is in an intermediary state")
            }
            AppRunningStatus::Stopped
            | AppRunningStatus::PartiallyRunning
            | AppRunningStatus::FullyRunning => {
                bail!("Application's containers already exist")
            }
        }

        let containers = self.sort_containers_by_deps();

        for (i, container) in containers.iter().enumerate() {
            info!(
                "> Creating container {} / {}: '{}' [{}]...",
                i + 1,
                containers.len(),
                container.name,
                container.id
            );

            let config = self.generate_container_config(container);

            docker::create_container(self.docker, config)
                .await
                .with_context(|| {
                    format!(
                        "Failed to create container '{}' for app '{}'",
                        container.name, container.app.name
                    )
                })?;
        }

        info!("> All containers were successfully created!");

        Ok(())
    }

    pub async fn start(&self) -> Result<()> {
        match self.status().await? {
            AppRunningStatus::NotCreated => bail!("Application's containers are not created yet"),
            AppRunningStatus::PartiallyCreated => {
                bail!("Some of the application's containers have not been created")
            }
            AppRunningStatus::Zombie => bail!("At least one container is in zombie mode"),
            AppRunningStatus::Intermediary => {
                bail!("At least one container is in an intermediary state")
            }
            AppRunningStatus::Stopped | AppRunningStatus::PartiallyRunning => {}
            AppRunningStatus::FullyRunning => return Ok(()),
        }

        let containers = self.sort_containers_by_deps();

        for (i, container) in containers.iter().enumerate() {
            info!(
                "> Starting container {} / {}: '{}' [{}]...",
                i + 1,
                containers.len(),
                container.name,
                container.id
            );

            docker::start_container(self.docker, &container.docker_container_name())
                .await
                .with_context(|| format!("Failed to start container '{}'", container.name))?;
        }

        info!("> All containers were successfully started!");

        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        let containers = self.list_existing_containers().await?;

        let docker = Arc::new(self.docker.clone());

        let tasks = containers
            .into_iter()
            .filter(|container| container.app_id == self.app.id)
            .map(|container| {
                let docker = Arc::clone(&docker);

                async move {
                    docker::stop_container(&docker, &container.docker_container_id)
                        .await
                        .with_context(|| {
                            format!("Failed to stop container '{}'", container.container_name)
                        })?;

                    Ok::<(), anyhow::Error>(())
                }
            })
            .collect::<Vec<_>>();

        try_join_all(tasks).await?;

        Ok(())
    }

    fn generate_container_config(&self, container: &AppContainer) -> ContainerCreationConfig {
        assert_eq!(
            container.app.id, self.app.id,
            "Assertion error: tried to generate a container's configuration for another app in runner"
        );

        let mut anon_volumes = vec![];
        let mut mounts = vec![];

        let container_identity = container.identity();

        for (name, volume) in &container.volumes {
            match volume {
                AppVolume::Disposable | AppVolume::Internal => {
                    anon_volumes.push(name.clone());
                }

                AppVolume::External {
                    container_path,
                    readonly,
                } => mounts.push(ContainerMount {
                    in_host: self.env.app_container_internal_volume_dir( &container_identity, name).to_str().expect("Internal error: normalized app container's internal volume path contains invalid UTF-8 characters").to_string(),
                    in_container: container_path.clone(),
                    readonly: *readonly,
                }),

                AppVolume::BindToPath {
                    real_path,
                    container_path,
                    readonly,
                } => mounts.push(ContainerMount {
                    in_host: real_path.clone(),
                    in_container: container_path.clone(),
                    readonly: *readonly
                }),
            }
        }

        ContainerCreationConfig {
            name: container.docker_container_name(),
            image: container.image.clone(),
            env: container.env_vars.clone(),
            anon_volumes,
            mounts,
            labels: HashMap::from([
                (APP_ID_LABEL.to_string(), container.app.id.to_string()),
                (APP_NAME_LABEL.to_string(), container.app.name.clone()),
                (CONTAINER_ID_LABEL.to_string(), container.id.to_string()),
                (CONTAINER_NAME_LABEL.to_string(), container.name.clone()),
            ]),
            restart_policy: ContainerRestartPolicy::UnlessStopped,
        }
    }

    fn sort_containers_by_deps(&self) -> Vec<&AppContainer> {
        let mut refs: Vec<_> = self.app.containers.iter().collect();

        refs.sort_by(|a, b| {
            if a.depends_on.contains(&b.name) {
                Ordering::Greater
            } else if b.depends_on.contains(&a.name) {
                Ordering::Less
            } else {
                a.name.cmp(&b.name)
            }
        });

        refs
    }
}

#[derive(Enum, Clone, Copy, PartialEq, Eq)]
pub enum AppRunningStatus {
    /// No container was created yet for this application
    NotCreated,

    /// Some containers (but not all) were created
    PartiallyCreated,

    /// All containers are created but at least one container is in a zombie state (e.g. 'dead')
    Zombie,

    /// All containers are created with no zombie but at least one container is in an intermediary state (e.g. 'restarting')
    Intermediary,

    /// All containers are created but stopped (exited)
    Stopped,

    /// At least one container is running and one is not, with no zombie or intermediary container
    PartiallyRunning,

    /// All containers are running
    FullyRunning,
}
