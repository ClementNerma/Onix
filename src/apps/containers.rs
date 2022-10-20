use std::{
    collections::{BTreeMap, HashSet},
    fmt::{Display, Formatter},
    hash::Hash,
    marker::PhantomData,
};

use anyhow::{bail, Result};
use async_graphql::{
    ComplexObject, InputObject, InputValueError, InputValueResult, Scalar, ScalarType,
    SimpleObject, Value,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{declare_id_type, docker::NAME_PREFIX, utils::time::get_now};

use super::{
    app::AppIdentity,
    volumes::{AppVolume, AppVolumeGraphQL},
    NAME_VALIDATOR,
};

#[derive(InputObject, Deserialize)]
pub struct AppContainerCreationInput {
    pub name: String,
    pub image: String,
    pub env_vars: BTreeMap<String, String>,
    pub port_bindings: BTreeMap<u16, u16>,
    pub volumes: BTreeMap<String, AppVolume>,
    pub depends_on: HashSet<String>,
}

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct AppContainer {
    pub app: AppIdentity,
    pub id: AppContainerId,
    pub name: String,
    pub image: String,
    pub env_vars: BTreeMap<String, String>,
    pub port_bindings: BTreeMap<u16, u16>,

    #[graphql(skip)]
    pub volumes: BTreeMap<String, AppVolume>,

    pub depends_on: HashSet<String>,
    created_on: OffsetDateTime,
}

#[ComplexObject]
impl AppContainer {
    pub async fn volumes(&self) -> BTreeMap<String, AppVolumeGraphQL> {
        self.volumes
            .iter()
            .map(|(name, volume)| (name.clone(), volume.encode_cloned()))
            .collect()
    }
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
        format!(
            "{NAME_PREFIX}_{}_{}_{}_{}",
            self.app.id, self.app.name, self.id, self.name
        )
    }
}

#[derive(SimpleObject, Serialize, Hash, Clone, PartialEq, Eq)]
pub struct AppContainerIdentity {
    pub id: AppContainerId,
    pub name: String,
    pub app: AppIdentity,

    #[graphql(skip)]
    __private: PhantomData<()>,
}

declare_id_type!(AppContainerId);
