use std::{collections::HashSet, marker::PhantomData};

use anyhow::{bail, Context, Result};
use async_graphql::{InputObject, SimpleObject};
use rand::Rng;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{declare_id_type, docker::ExistingContainer, utils::time::get_now};

use super::{containers::AppContainer, existing_containers::ExistingAppContainer, AppTemplate};

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
#[graphql(complex)]
pub struct App {
    pub id: AppId,
    pub name: String,
    pub containers: Vec<AppContainer>,
    created_on: OffsetDateTime,
}

impl App {
    pub fn new(input: AppTemplate) -> Result<Self> {
        if input.name.trim().is_empty() {
            bail!("Please provide a non-empty name");
        }

        #[deny(unused_variables)]
        let AppTemplate { name, containers } = input;

        let mut app = Self {
            id: AppId(rand::thread_rng().gen()),
            name,
            containers: vec![],
            created_on: get_now(),
        };

        let containers = containers
            .into_iter()
            .map(|input| AppContainer::new(app.identity(), input))
            .collect::<Result<Vec<_>, _>>()
            .with_context(|| {
                format!("Failed to create containers for application '{}'", app.name)
            })?;

        app.add_containers(containers)?;

        Ok(app)
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

    pub fn add_containers(&mut self, containers: Vec<AppContainer>) -> Result<()> {
        let container_names = self
            .containers
            .iter()
            .chain(containers.iter())
            .map(|container| container.name.clone())
            .collect::<HashSet<_>>();

        for container in containers {
            if self.get_container(&container.name).is_some() {
                bail!(
                    "This application already has a container with the '{}' name",
                    container.name
                );
            }

            for dep in &container.depends_on {
                if !container_names.contains(dep) {
                    bail!(
                        "Container '{}' depends on unknown container '{}'",
                        container.name,
                        dep
                    );
                }
            }

            self.containers.push(container.clone());
        }

        Ok(())
    }

    pub fn decode_container(
        &self,
        container: ExistingContainer,
    ) -> Result<Option<ExistingAppContainer>> {
        let container = ExistingAppContainer::decode(container)?;

        match container {
            Some(container) if container.app_id == self.id => Ok(Some(container)),
            _ => Ok(None),
        }
    }

    pub fn to_template(self) -> AppTemplate {
        #[deny(unused_variables)]
        let Self {
            name,
            containers,

            id: _,
            created_on: _,
        } = self;

        AppTemplate {
            name,
            containers: containers
                .into_iter()
                .map(AppContainer::to_template)
                .collect(),
        }
    }
}

declare_id_type!(AppId);

#[derive(SimpleObject, InputObject, Serialize, Deserialize, Hash, Clone, PartialEq, Eq)]
#[graphql(input_name_suffix = "Input")]
pub struct AppIdentity {
    pub id: AppId,
    pub name: String,

    #[graphql(skip)]
    #[serde(skip_serializing, skip_deserializing)]
    __private: PhantomData<()>,
}
