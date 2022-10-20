use anyhow::Context as _;
use async_graphql::{Context, Object};

use crate::{
    apps::{App, AppCreationInput},
    utils::graphql::{Result, Void},
};

use super::{graphql::get_state, state::get_runner_for};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_app(&self, ctx: &Context<'_>, input: AppCreationInput) -> Result<App> {
        let apps = &mut get_state(ctx).await.user_data.apps;

        if apps.iter().any(|app| app.name == input.name) {
            Err("An application already exists with the provided name")?;
        }

        let app = App::new(input).context("Failed to create the application")?;

        Ok(app)
    }

    async fn start_app(&self, ctx: &Context<'_>, app_id: u64) -> Result<Void> {
        let state = &get_state(ctx).await;

        let runner = get_runner_for(&state, app_id).await?;

        runner.start().await.map(|()| Void).map_err(Into::into)
    }

    async fn stop_app(&self, ctx: &Context<'_>, app_id: u64) -> Result<Void> {
        let state = &get_state(ctx).await;

        let runner = get_runner_for(&state, app_id).await?;

        runner.stop().await.map(|()| Void).map_err(Into::into)
    }
}
