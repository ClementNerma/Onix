use async_graphql::Object;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn server_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}
