//! GraphQL interfaces
use crate::Share;
use actix_web::{error::InternalError, http::StatusCode, web, Error, HttpResponse};
use juniper::{graphql_object, EmptyMutation, EmptySubscription, RootNode};
use juniper_actix::{graphiql_handler, graphql_handler, playground_handler};

/// Ceres Queries
pub struct Query;

#[graphql_object(context = Share)]
impl Query {
    fn version() -> String {
        "1.0".to_string()
    }
}

/// GraphQL Schema
pub type Schema = RootNode<'static, Query, EmptyMutation<Share>, EmptySubscription<Share>>;

/// Generate schema
pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::<Share>::new(),
        EmptySubscription::<Share>::new(),
    )
}

/// GraphiQL Route
pub async fn graphiql_route() -> Result<HttpResponse, Error> {
    graphiql_handler("/graphgl", None).await
}

/// GraphQL playground
pub async fn playground_route() -> Result<HttpResponse, Error> {
    playground_handler("/graphgl", None).await
}

/// GraphQL Route
pub async fn graphql_route(
    req: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
    schema: web::Data<Schema>,
) -> Result<HttpResponse, Error> {
    let context = Share::new("ws://192.168.2.142:4242").await.map_err(|_| {
        Error::from(InternalError::new(
            "".to_string(),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    })?;
    graphql_handler(&schema, &context, req, payload).await
}
