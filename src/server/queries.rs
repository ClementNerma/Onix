use async_graphql::{Context, Object};

use crate::{
    apps::{App, AppRunningStatus},
    docker,
    utils::graphql::{CustomGraphQLError, Result},
};

use super::{graphql::get_state, state::get_runner_for};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn server_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    async fn docker_version(&self, ctx: &Context<'_>) -> Result<Option<String>> {
        docker::docker_version(&get_state(ctx).await.docker)
            .await
            .map_err(Into::into)
    }

    async fn apps(&self, ctx: &Context<'_>) -> Vec<App> {
        get_state(ctx).await.user_data.apps.clone()
    }

    async fn app(&self, ctx: &Context<'_>, app_id: u64) -> Result<App, &'static str> {
        get_state(ctx)
            .await
            .user_data
            .apps
            .iter()
            .find(|app| app.id == app_id)
            .cloned()
            .ok_or("Provided application ID was not found")
    }

    async fn app_status(&self, ctx: &Context<'_>, app_id: u64) -> Result<AppRunningStatus> {
        let state = &get_state(ctx).await;

        let runner = get_runner_for(&state, app_id).await?;

        runner.status().await.map_err(CustomGraphQLError::from)
    }
}
