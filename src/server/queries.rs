use async_graphql::Object;

use crate::{
    docker,
    utils::graphql::{format_err, Result},
};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn server_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    async fn docker_version(&self) -> Result<String> {
        docker::docker_version().map_err(format_err)
    }

    async fn docker_compose_version(&self) -> Result<String> {
        docker::docker_compose_version().map_err(format_err)
    }
}
