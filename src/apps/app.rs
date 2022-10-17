use std::collections::{BTreeMap, HashSet};

use anyhow::{bail, Result};
use async_graphql::SimpleObject;
use once_cell::sync::Lazy;
use pomsky_macro::pomsky;
use regex::Regex;
use serde::Serialize;
use time::OffsetDateTime;

use crate::{graphql_enum, utils::time::get_now};

#[derive(SimpleObject, Serialize)]
pub struct App {
    pub name: String,
    pub stash: BTreeMap<String, AppContainer>,
    created_on: OffsetDateTime,
}

impl App {
    pub fn new(name: String, stash: BTreeMap<String, AppContainer>) -> Result<Self> {
        if APP_NAME_VALIDATOR.is_match(&name) {
            bail!("Invalid name, please follow regex: {APP_NAME_VALIDATOR:?}");
        }

        Ok(Self {
            name,
            stash,
            created_on: get_now(),
        })
    }
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

static APP_NAME_VALIDATOR: Lazy<Regex> = Lazy::new(|| {
    Regex::new(pomsky!(
        Start ['a'-'z' 'A'-'Z' '0'-'9' '-' '_']+ End
    ))
    .unwrap()
});
