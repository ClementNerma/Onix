use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{self, IntoResponse},
    Extension,
};

use super::{mutations::MutationRoot, queries::QueryRoot, state::WrappedState};

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
