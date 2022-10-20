use async_graphql::{http::GraphiQLSource, Context, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    Extension,
};
use tokio::sync::MutexGuard;

use super::{
    mutations::MutationRoot,
    queries::QueryRoot,
    state::{State, WrappedState},
};

pub type AppSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphiql(Extension(state): Extension<WrappedState>) -> impl IntoResponse {
    response::Html(
        GraphiQLSource::build()
            // TODO: configure host IP
            .endpoint(&format!("http://localhost:{}", state.lock().await.port))
            .finish(),
    )
}

pub async fn get_state<'a>(context: &'a Context<'_>) -> MutexGuard<'a, State> {
    context
        .data::<WrappedState>()
        .expect("Assertion error: GraphQL context does not have the expected type")
        .lock()
        .await
}
