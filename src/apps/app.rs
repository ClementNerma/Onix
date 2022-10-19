use std::{
    collections::{hash_map::DefaultHasher, BTreeMap, HashSet},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use anyhow::{bail, Result};
use async_graphql::{ComplexObject, SimpleObject};
use once_cell::sync::Lazy;
use pomsky_macro::pomsky;
use regex::Regex;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{graphql_enum, utils::time::get_now};

#[derive(SimpleObject, Serialize)]
pub struct App {
    pub id: u64,
    pub name: String,
    pub containers: Vec<AppContainer>,
    created_on: OffsetDateTime,
}

impl App {
    pub fn new(name: String, stash: Vec<AppContainer>) -> Result<Self> {
        if APP_NAME_VALIDATOR.is_match(&name) {
            bail!("Invalid name, please follow regex: {APP_NAME_VALIDATOR:?}");
        }

        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);

        Ok(Self {
            id: hasher.finish(),
            name,
            containers: stash,
            created_on: get_now(),
        })
    }

    pub fn identity(&self) -> AppIdentity {
        AppIdentity {
            id: self.id,
            name: self.name.clone(),
            __private: PhantomData,
        }
    }

    pub fn get_container(&self, name: &str) -> Option<&AppContainer> {
        self.containers
            .iter()
            .find(|container| container.name == name)
    }
}

#[derive(SimpleObject, Serialize, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AppIdentity {
    pub id: u64,
    pub name: String,

    #[graphql(skip)]
    __private: PhantomData<()>,
}

#[derive(SimpleObject, Serialize)]
pub struct AppContainer {
    pub id: u64,
    pub name: String,
    pub app: AppIdentity,
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
    pub fn identity(&self) -> AppContainerIdentity {
        AppContainerIdentity {
            id: self.id,
            name: self.name.clone(),
            app: self.app.clone(),
            __private: PhantomData,
        }
    }
}

#[derive(SimpleObject, Serialize, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AppContainerIdentity {
    pub id: u64,
    pub name: String,
    pub app: AppIdentity,

    #[graphql(skip)]
    __private: PhantomData<()>,
}

graphql_enum!(
    #[derive(Serialize, Deserialize)]
    pub enum AppVolume {
        /// Volume that could be dropped without any real datal loss
        /// (e.g. cache or unimportant configuration files)
        Disposable,

        /// Internal volume used to store data which does not need to be modifiable
        /// by the end user (non-disposable)
        Internal,

        /// External volume stored in an accessible filesystem
        External {
            container_path: String,
            readonly: bool,
        },

        /// Binding to a real directory
        BindToPath {
            real_path: String,
            container_path: String,
            readonly: bool,
        },
        // TODO: /// Binding to a global path
        // #[derive(PartialEq, Eq, Hash)]
        // GlobalPath {
        //     global_path_id: String,
        //     readonly: bool,
        // },
    }
);

static APP_NAME_VALIDATOR: Lazy<Regex> = Lazy::new(|| {
    Regex::new(pomsky!(
        Start ['a'-'z' 'A'-'Z' '0'-'9' '-' '_']+ End
    ))
    .unwrap()
});
