#[cfg(test)]
#[path = "graphql.test.rs"]
mod tests;

use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Json, ServiceConfig},
    HttpResponse, Result,
};
use juniper::http::{playground, GraphQLRequest};
use std::sync::Arc;

use crate::api::schema::{Context, Schema};
use crate::database::{self, pool::Pool};

#[actix_web::get("/graphql")]
async fn graphql_get_handler() -> Result<HttpResponse> {
    let body = playground::playground_source("", None);

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}

#[actix_web::post("/graphql")]
async fn graphql_post_handler(
    pool: Data<Pool>,
    schema: Data<Arc<Schema>>,
    request: Json<GraphQLRequest>,
) -> Result<HttpResponse> {
    let connection = database::connect(&pool)?;
    let context = Context::new(connection);

    let response = request.execute(&schema, &context).await;
    let body = serde_json::to_string(&response).map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

pub fn configure(config: &mut ServiceConfig) {
    #[cfg(debug_assertions)]
    config.service(graphql_get_handler);
    config.service(graphql_post_handler);
}
