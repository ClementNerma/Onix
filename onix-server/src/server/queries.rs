use async_graphql::{ComplexObject, Context, Object};

use crate::{
    apps::{App, AppContainer, AppId, AppRunningStatus, ExistingAppContainer},
    docker,
    utils::graphql::{CustomGraphQLError, Result},
};

use super::state::{get_runner_for, get_state};

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
        get_state(ctx).await.user_data().apps.clone()
    }

    async fn app(&self, ctx: &Context<'_>, id: AppId) -> Result<App, &'static str> {
        get_state(ctx)
            .await
            .user_data()
            .apps
            .iter()
            .find(|app| app.id == id)
            .cloned()
            .ok_or("Provided application ID was not found")
    }

    async fn app_status(&self, ctx: &Context<'_>, id: AppId) -> Result<AppRunningStatus> {
        let state = &get_state(ctx).await;
        let runner = get_runner_for(&state, id).await?;

        runner.status().await.map_err(CustomGraphQLError::from)
    }
}

#[ComplexObject]
impl App {
    async fn fetched_status(&self, ctx: &Context<'_>) -> Result<AppRunningStatus> {
        let state = &get_state(ctx).await;
        let runner = get_runner_for(&state, self.id).await?;

        runner.status().await.map_err(CustomGraphQLError::from)
    }

    async fn generate_template(&self, ctx: &Context<'_>) -> Result<String> {
        let state = &get_state(ctx).await;
        let runner = get_runner_for(&state, self.id).await?;

        Ok(serde_yaml::to_string(&runner.generate_app_template())
            .expect("Assertion error: failed to convert application template to YAML string"))
    }
}

#[ComplexObject]
impl AppContainer {
    async fn docker_container(&self, ctx: &Context<'_>) -> Result<Option<ExistingAppContainer>> {
        let state = &get_state(ctx).await;
        let runner = get_runner_for(&state, self.app.id).await?;

        runner
            .get_container_infos(&self)
            .await
            .map_err(CustomGraphQLError::from)
    }
}
