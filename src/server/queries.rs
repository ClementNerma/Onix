use async_graphql::{Context, Object};

use crate::{
    docker,
    utils::graphql::{format_err, Result},
};

use super::graphql::get_state;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn server_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    async fn docker_version(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        docker::docker_version(&get_state(ctx).docker)
            .await
            .map_err(format_err)
    }
}
