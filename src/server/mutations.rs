use async_graphql::{Context, Object};

use crate::{
    apps::{App, AppCreationInput},
    utils::graphql::Result,
};

use super::graphql::get_state;

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
}
