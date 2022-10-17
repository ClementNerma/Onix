use std::collections::HashMap;

use anyhow::Result;
use bollard::{container::Config, Docker};

use super::app::{App, AppContainer, AppVolume, AppVolumeGraphQL};

pub struct AppRunner<'a, 'b> {
    app: &'a App,
    docker: &'b Docker,
}

impl<'a, 'b> AppRunner<'a, 'b> {
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

        // for container in &self.app.stash {
        //     docker::create_container(self.docker, self.app.name, Config {}).await?;
        // }

        Ok(())
    }
}

impl AppContainer {
    pub fn generate_container_config(&self) -> Config<String> {
        let env = self
            .env_vars
            .iter()
            .map(|(name, value)| format!("{name}={value}"))
            .collect();

        let volumes = HashMap::new();

        for volume in &self.volumes {
            match volume.decode_cloned() {
                AppVolume::Disposable => todo!(),
                AppVolume::Internal => todo!(),
                AppVolume::External {
                    accessible_path,
                    readonly,
                } => todo!(),
                AppVolume::BindToPath {
                    real_path,
                    readonly,
                } => todo!(),
                AppVolume::GlobalPath {
                    global_path_id,
                    readonly,
                } => todo!(),
            }
        }

        Config {
            env: Some(env),
            image: Some(self.image.clone()),
            volumes: Some(volumes),
            ..Config::default()
        }
    }
}
