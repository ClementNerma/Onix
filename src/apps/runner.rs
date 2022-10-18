use anyhow::Result;
use bollard::Docker;

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
        todo!()
    }

    pub async fn stop(&self) -> Result<()> {
        todo!()
    }

    pub async fn create_containers(&self) -> Result<()> {
        if self.is_partially_running().await? {
            self.stop().await?;
        }

        for container in &self.app.stash {
            docker::create_container(self.docker, self.generate_container_config(container))
                .await?;
        }

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
}
