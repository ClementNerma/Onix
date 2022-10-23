use std::{
    collections::{BTreeMap, HashSet},
    hash::Hash,
    marker::PhantomData,
};

use anyhow::{bail, Result};
use async_graphql::{InputObject, SimpleObject};
use rand::Rng;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{
    declare_id_type,
    docker::{ContainerPortBinding, NAME_PREFIX},
    utils::time::get_now,
};

use super::{app::AppIdentity, volumes::AppVolume, NAME_VALIDATOR};

#[derive(InputObject, Deserialize)]
pub struct AppContainerCreationInput {
    pub name: String,
    pub image: String,
    pub env_vars: BTreeMap<String, String>,
    pub port_bindings: Vec<ContainerPortBinding>,
    pub volumes: Vec<AppVolume>,
    pub depends_on: HashSet<String>,
}

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
#[graphql(complex)]
pub struct AppContainer {
    pub app: AppIdentity,
    pub id: AppContainerId,
    pub name: String,
    pub image: String,
    pub env_vars: BTreeMap<String, String>,
    pub port_bindings: Vec<ContainerPortBinding>,
    pub volumes: Vec<AppVolume>,
    pub depends_on: HashSet<String>,
    created_on: OffsetDateTime,
}

impl AppContainer {
    pub fn new(app: AppIdentity, input: AppContainerCreationInput) -> Result<Self> {
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

        if input.env_vars.keys().any(|name| name.trim().is_empty()) {
            bail!("Please provide a non-empty for all environment variables");
        }

        if let Some(name) = input
            .env_vars
            .keys()
            .find(|name| !NAME_VALIDATOR.is_match(&name))
        {
            bail!(
                "Invalid environment variable name provided '{name}', please follow regex: {}",
                NAME_VALIDATOR.as_str()
            );
        }

        if let Some((name, _)) = input
            .env_vars
            .iter()
            .find(|(_, value)| value.trim().is_empty())
        {
            bail!("Please provide a value for the '{name}' environment variable or remove this variable");
        }

        if let Some((binding_a, binding_b)) =
            ContainerPortBinding::find_collision(&input.port_bindings)
        {
            bail!(
                "Collision detected between two bindings' ports: [{binding_a}] and [{binding_b}]"
            );
        }

        #[deny(unused_variables)]
        let AppContainerCreationInput {
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
