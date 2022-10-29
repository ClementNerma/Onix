use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use anyhow::{bail, Result};
use async_graphql::SimpleObject;
use rand::Rng;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{
    declare_id_type,
    docker::{ContainerEnvironmentVar, ContainerPortBinding, NAME_PREFIX},
    utils::time::get_now,
};

use super::{app::AppIdentity, AppContainerTemplate, AppVolume, NAME_VALIDATOR};

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
#[graphql(complex)]
pub struct AppContainer {
    pub app: AppIdentity,
    pub id: AppContainerId,
    pub name: String,
    pub image: String,
    pub env_vars: Vec<ContainerEnvironmentVar>,
    pub port_bindings: Vec<ContainerPortBinding>,
    pub volumes: Vec<AppVolume>,
    pub depends_on: Vec<String>,
    created_on: OffsetDateTime,
}

impl AppContainer {
    pub fn new(app: AppIdentity, input: AppContainerTemplate) -> Result<Self> {
        if input.name.trim().is_empty() {
            bail!("Please provide a non-empty name for this container")
        }

        if !NAME_VALIDATOR.is_match(&input.name) {
            bail!(
                "Invalid container name provided, please follow regex: {}",
                NAME_VALIDATOR.as_str()
            );
        }

        if input.image.trim().is_empty() {
            bail!("Please provide a non-empty image name");
        }

        for ContainerEnvironmentVar {
            ref name,
            ref value,
        } in &input.env_vars
        {
            if name.is_empty() {
                bail!("Please provide a non-empty for all environment variables");
            }

            if !NAME_VALIDATOR.is_match(&name) {
                bail!(
                    "Invalid environment variable name provided '{name}', please follow regex: {}",
                    NAME_VALIDATOR.as_str()
                );
            }

            if value.trim().is_empty() {
                bail!("Please provide a value for the '{name}' environment variable or remove this variable");
            }
        }

        for dependance in &input.depends_on {
            if input
                .depends_on
                .iter()
                .filter(|dep| dep == &dependance)
                .count()
                > 1
            {
                bail!("Dependance '{}' was specified twice", dependance);
            }
        }

        if let Some((binding_a, binding_b)) =
            ContainerPortBinding::find_collision(&input.port_bindings)
        {
            bail!(
                "Collision detected between two bindings' ports: [{binding_a}] and [{binding_b}]"
            );
        }

        #[deny(unused_variables)]
        let AppContainerTemplate {
            name,
            image,
            env_vars,
            port_bindings,
            volumes,
            depends_on,
        } = input;

        Ok(Self {
            app,
            id: AppContainerId(rand::thread_rng().gen()),
            name,
            image,
            env_vars,
            port_bindings,
            volumes,
            depends_on,
            created_on: get_now(),
        })
    }

    pub fn identity(&self) -> AppContainerIdentity {
        AppContainerIdentity {
            id: self.id,
            name: self.name.clone(),
            app: self.app.clone(),
            __private: PhantomData,
        }
    }

    pub fn docker_container_name(&self) -> String {
        format!("{NAME_PREFIX}{}_{}", self.app.id.encode(), self.id.encode())
    }

    pub fn get_docker_volume_name(&self, volume: &str) -> String {
        let mut hasher = DefaultHasher::new();
        volume.hash(&mut hasher);

        format!(
            "{NAME_PREFIX}{}_{}_{}",
            self.app.id.encode(),
            self.id.encode(),
            base62::encode(hasher.finish())
        )
    }

    pub fn to_template(self) -> AppContainerTemplate {
        #[deny(unused_variables)]
        let Self {
            name,
            image,
            env_vars,
            port_bindings,
            volumes,
            depends_on,

            app: _,
            id: _,
            created_on: _,
        } = self;

        AppContainerTemplate {
            name,
            image,
            env_vars,
            port_bindings,
            volumes,
            depends_on,
        }
    }
}

#[derive(SimpleObject, Serialize, Hash, Clone, PartialEq, Eq)]
pub struct AppContainerIdentity {
    pub id: AppContainerId,
    pub name: String,
    pub app: AppIdentity,

    #[graphql(skip)]
    #[serde(skip_serializing, skip_deserializing)]
    __private: PhantomData<()>,
}

declare_id_type!(AppContainerId);
