mod graphql;
mod mutations;
mod queries;
mod state;

use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use async_graphql::{EmptySubscription, Schema};
use axum::{extract::Extension, routing::get, Router, Server};
use log::info;

pub use state::StateConfig;

use crate::server::{
    graphql::{graphiql, graphql_handler},
    mutations::MutationRoot,
    queries::QueryRoot,
    state::State,
};

pub async fn start(config: StateConfig) -> Result<()> {
    let state = Arc::new(State::new(config));

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription)
        .data(Arc::clone(&state))
        .finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
        .layer(Extension(Arc::clone(&state)));

    let addr = format!("{}:{}", state.address, state.port)
        .parse()
        .context("Failed to parse listening address")?;

    info!("Starting the server...");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|err| anyhow!("Failed to run server: {err}"))
}
