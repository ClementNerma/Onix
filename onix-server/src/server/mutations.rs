use anyhow::Context as _;
use async_graphql::{Context, Object};

use crate::{
    apps::{App, AppCreationInput, AppId},
    utils::graphql::{Result, Void},
};

use super::state::{get_runner_for, get_state};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_app(&self, ctx: &Context<'_>, input: AppCreationInput) -> Result<App> {
        let mut state = get_state(ctx).await;
        let apps = &mut state.user_data_mut().apps;

        if apps.iter().any(|app| app.name == input.name) {
            Err("An application already exists with the provided name")?;
        }

        let app = App::new(input).context("Failed to create the application")?;

        apps.push(app.clone());

        Ok(app)
    }

    async fn create_app_containers(&self, ctx: &Context<'_>, id: AppId) -> Result<Void> {
        let state = &get_state(ctx).await;

        let runner = get_runner_for(&state, id).await?;

        runner
            .create_containers()
            .await
            .map(Into::into)
            .map_err(Into::into)
    }

    async fn start_app(&self, ctx: &Context<'_>, id: AppId) -> Result<Void> {
        let state = &get_state(ctx).await;

        let runner = get_runner_for(&state, id).await?;

        runner.start().await.map(Into::into).map_err(Into::into)
    }

    async fn stop_app(&self, ctx: &Context<'_>, id: AppId) -> Result<Void> {
        let state = &get_state(ctx).await;

        let runner = get_runner_for(&state, id).await?;

        runner.stop().await.map(Into::into).map_err(Into::into)
    }

    async fn remove_app_containers(&self, ctx: &Context<'_>, id: AppId) -> Result<Void> {
        let state = &get_state(ctx).await;

        let runner = get_runner_for(&state, id).await?;

        runner
            .remove_containers()
            .await
            .map(Into::into)
            .map_err(Into::into)
    }
}
