use std::sync::Arc;

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    Extension,
};

use super::{queries::QueryRoot, state::State};

type AppSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql(Extension(state): Extension<Arc<State>>) -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            // TODO: configure host IP
            .endpoint(&format!("http://localhost:{}", state.port))
            .finish(),
    )
}
