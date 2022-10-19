use std::path::PathBuf;

use super::{app::AppIdentity, containers::AppContainerIdentity};

pub struct AppRunnerConfig {
    pub data_dir: PathBuf,
}

pub struct AppRunnerEnvironment {
    pub(super) data_dir: PathBuf,
    pub(super) apps_dir: PathBuf,
}

impl From<AppRunnerConfig> for AppRunnerEnvironment {
    fn from(config: AppRunnerConfig) -> AppRunnerEnvironment {
        #[forbid(unused_variables)]
        let AppRunnerConfig { data_dir } = config;

        Self {
            apps_dir: data_dir.join("apps"),

            data_dir,
        }
    }
}

impl AppRunnerEnvironment {
    pub fn app_dir(&self, app: &AppIdentity) -> PathBuf {
        self.apps_dir.join(&format!("{}-{}", app.name, app.id))
    }

    pub fn app_containers_dir(&self, app: &AppIdentity) -> PathBuf {
        self.app_dir(app).join("containers")
    }

    pub fn app_container_dir(&self, container: &AppContainerIdentity) -> PathBuf {
        self.app_containers_dir(&container.app)
            .join(&format!("{}-{}", container.name, container.id))
    }

    pub fn app_container_internal_volumes_dir(&self, container: &AppContainerIdentity) -> PathBuf {
        self.app_container_dir(container).join("internal-volumes")
    }

    pub fn app_container_internal_volume_dir(
        &self,
        container: &AppContainerIdentity,
        volume_name: &str,
    ) -> PathBuf {
        self.app_container_internal_volumes_dir(container)
            .join(volume_name)
    }
}
