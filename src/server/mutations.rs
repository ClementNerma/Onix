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
            return Err("An application already exists with the provided name".into());
        }

        let app =
            App::new(input).map_err(|err| format!("Failed to create the application: {err}"))?;

        Ok(app)
    }
}
