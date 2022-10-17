use std::collections::{BTreeMap, HashSet};

use async_graphql::SimpleObject;
use serde::Serialize;

use crate::graphql_enum;

#[derive(SimpleObject, Serialize)]
pub struct App {
    pub name: String,
    pub stash: BTreeMap<String, AppContainer>,
}

#[derive(SimpleObject, Serialize)]
pub struct AppContainer {
    pub name: String,
    pub image: String,
    pub env_vars: BTreeMap<String, String>,
    pub port_bindings: BTreeMap<u16, u16>,
    pub volumes: Vec<AppVolumeGraphQL>,
    pub depends_on: HashSet<String>,
}

graphql_enum!(
    #[derive(PartialEq, Eq, Hash)]
    pub enum AppVolume {
        /// Volume that could be dropped without any real datal loss
        /// (e.g. cache or unimportant configuration files)
        #[derive(PartialEq, Eq, Hash)]
        Disposable { internal_path: String },

        /// Internal volume used to store data which does not need to be modifiable
        /// by the end user (non-disposable)
        #[derive(PartialEq, Eq, Hash)]
        Internal { internal_path: String },

        /// External volume stored in an accessible filesystem
        #[derive(PartialEq, Eq, Hash)]
        External {
            accessible_path: String,
            readonly: bool,
        },

        /// Binding to a real directory
        #[derive(PartialEq, Eq, Hash)]
        BindToPath { real_path: String, readonly: bool },

        /// Binding to a global path
        #[derive(PartialEq, Eq, Hash)]
        GlobalPath {
            global_path_id: String,
            readonly: bool,
        },
    }
);
