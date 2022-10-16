mod graphql;
mod queries;
mod state;

use std::sync::Arc;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use axum::{extract::Extension, routing::get, Router, Server};
use log::info;

use crate::server::{
    graphql::{graphiql, graphql_handler},
    queries::QueryRoot,
    state::State,
};

pub use self::state::StateConfig;

pub async fn start(config: StateConfig) -> Result<(), String> {
    let state = Arc::new(State::new(config));

    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(Arc::clone(&state))
        .finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema))
        .layer(Extension(Arc::clone(&state)));

    let addr = format!("{}:{}", state.address, state.port)
        .parse()
        .map_err(|err| format!("Failed to parse listening address: {err}"))?;

    info!("Starting the server...");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|err| format!("Server failed: {err}"))
}
