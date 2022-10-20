mod graphql;
mod mutations;
mod queries;
mod state;

use anyhow::{anyhow, Context, Result};
use async_graphql::EmptySubscription;
use axum::{extract::Extension, routing::get, Router, Server};
use log::info;

pub use state::StateConfig;

use crate::server::{
    graphql::{graphiql, graphql_handler, AppSchema},
    mutations::MutationRoot,
    queries::QueryRoot,
    state::WrappedState,
};

pub async fn start(config: StateConfig) -> Result<()> {
    let state = WrappedState::new(config);

    let schema = AppSchema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(state.clone())
        .finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
        .layer(Extension(state.clone()));

    let addr = {
        let state = state.lock().await;
        format!("{}:{}", state.address, state.port)
    };

    let addr = addr.parse().context("Failed to parse listening address")?;

    info!("Starting the server...");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|err| anyhow!("Failed to run server: {err}"))
}
