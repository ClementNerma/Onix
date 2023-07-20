mod graphql;
mod logger;
mod mutations;
mod queries;
mod state;
mod user_data;

use anyhow::{anyhow, Context, Result};
use async_graphql::EmptySubscription;
use axum::{extract::Extension, http::Method, routing::get, Router, Server};
use log::info;
use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer};

pub use state::StateConfig;

use crate::server::{
    graphql::{graphiql, graphql_handler, AppSchema},
    logger::Logger,
    mutations::MutationRoot,
    queries::QueryRoot,
    state::WrappedState,
    user_data::user_data_saver,
};

pub async fn start(config: StateConfig) -> Result<()> {
    // TODO: Restrict origin allowing to the same host
    // TODO: Investigate if headers should be limited to a whitelist
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(AllowOrigin::any())
        .allow_headers(AllowHeaders::any());

    let state = WrappedState::new(config);

    let schema = AppSchema::build(QueryRoot, MutationRoot, EmptySubscription)
        .extension(Logger)
        .data(state.clone())
        .finish();

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(cors)
        .layer(Extension(schema))
        .layer(Extension(state.clone()));

    let addr = {
        let state = state.lock().await;
        format!("{}:{}", state.address, state.port)
    };

    let addr = addr.parse().context("Failed to parse listening address")?;

    info!("Starting the user data saver thread...");

    let state_for_saver = state.clone();

    tokio::spawn(async move {
        user_data_saver(state_for_saver).await;
        panic!("Assertion error: user data saver loop unexpectedly exited!");
    });

    info!("Starting the server on {addr}...");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|err| anyhow!("Failed to run server: {err}"))
}
