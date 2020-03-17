use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Json, ServiceConfig},
    Error, HttpResponse,
};
use juniper::http::{playground, GraphQLRequest};
use serde_json;
use std::sync::Arc;

use crate::api::schema::{Context, Schema};

#[actix_web::get("/graphql")]
async fn graphql_get_handler() -> HttpResponse {
    let body = playground::playground_source("");

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body)
}

#[actix_web::post("/graphql")]
async fn graphql_post_handler(
    schema: Data<Arc<Schema>>,
    data: Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let response = data.execute(&schema, &Context {});
    let body = serde_json::to_string(&response).map_err(ErrorInternalServerError)?;

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body))
}

pub fn configure(config: &mut ServiceConfig) {
    if cfg!(debug_assertions) {
        config.service(graphql_get_handler);
    }

    config.service(graphql_post_handler);
}
