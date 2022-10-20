use std::{collections::HashSet, marker::PhantomData};

use anyhow::{bail, Context, Result};
use async_graphql::{InputObject, SimpleObject};
use rand::Rng;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::utils::time::get_now;

use super::{
    containers::{AppContainer, AppContainerCreationInput},
    NAME_VALIDATOR,
};

#[derive(InputObject, Deserialize)]
pub struct AppCreationInput {
    pub name: String,
    pub containers: Vec<AppContainerCreationInput>,
}

#[derive(SimpleObject, Serialize, Deserialize, Clone)]
pub struct App {
    pub id: u64,
    pub name: String,
    pub containers: Vec<AppContainer>,
    created_on: OffsetDateTime,
}

impl App {
    pub fn new(input: AppCreationInput) -> Result<Self> {
        if !NAME_VALIDATOR.is_match(&input.name) {
            bail!(
                "Invalid name, please follow regex: {}",
                NAME_VALIDATOR.as_str()
            );
        }

        #[deny(unused_variables)]
        let AppCreationInput { name, containers } = input;

        let mut app = Self {
            id: rand::thread_rng().gen(),
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
}

#[derive(
    SimpleObject, InputObject, Serialize, Deserialize, Hash, Clone, PartialEq, Eq, PartialOrd, Ord,
)]
#[graphql(input_name = "AppIdentityInput")]
pub struct AppIdentity {
    pub id: u64,
    pub name: String,

    #[graphql(skip)]
    __private: PhantomData<()>,
}
