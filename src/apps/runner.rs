use std::{cmp::Ordering, sync::Arc};

use anyhow::{Context, Result};
use bollard::Docker;
use futures::future::{join_all, try_join, try_join_all};
use log::info;

use crate::docker::{self, ContainerCreationConfig, ContainerMount};

use super::{
    app::{App, AppContainer, AppVolume},
    env::AppRunnerEnvironment,
};

pub struct AppRunner<'a, 'b, 'c> {
    env: &'a AppRunnerEnvironment,
    app: &'b App,
    docker: &'c Docker,
}

impl<'a, 'b, 'c> AppRunner<'a, 'b, 'c> {
    pub async fn is_partially_running(&self) -> Result<bool> {
        let containers = docker::list_containers(self.docker).await?;

        Ok(containers
            .into_iter()
            .any(|container| container.app_id == self.app.id && container.status.running_like()))
    }

    pub async fn stop(&self) -> Result<()> {
        let containers = docker::list_containers(self.docker).await?;

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

    pub async fn create_containers(&self) -> Result<()> {
        info!(
            "Creating containers for application '{}' [{}]...",
            self.app.name, self.app.id
        );

        if self.is_partially_running().await? {
            self.stop().await?;
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

            docker::create_container(self.docker, self.generate_container_config(container))
                .await
                .with_context(|| format!("Failed to start container '{}'", container.name))?;
        }

        info!("> All containers were successfully created!");

        Ok(())
    }

    fn generate_container_config(&self, container: &AppContainer) -> ContainerCreationConfig {
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
            name: container.name.clone(),
            image: container.image.clone(),
            env: container.env_vars.clone(),
            anon_volumes,
            mounts,
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
