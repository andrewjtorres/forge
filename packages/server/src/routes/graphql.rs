#[cfg(test)]
#[path = "graphql.test.rs"]
mod tests;

use actix_web::{
    error::{ErrorBadRequest, ErrorUnsupportedMediaType},
    web::{Data, Payload, Query},
    FromRequest, HttpMessage, HttpRequest, HttpResponse, Result,
};
use juniper::http::{playground, GraphQLBatchRequest, GraphQLRequest};
use serde::Deserialize;

use crate::api::schema::{Context, Schema};
use crate::database::{self, pool::Pool};

#[derive(Deserialize, Clone, PartialEq, Debug)]
struct GraphQLRequestSearchParams {
    #[serde(rename = "operationName")]
    operation_name: Option<String>,
    query: String,
    variables: Option<String>,
}

impl From<GraphQLRequestSearchParams> for GraphQLRequest {
    fn from(params: GraphQLRequestSearchParams) -> Self {
        let GraphQLRequestSearchParams {
            operation_name,
            query,
            ..
        } = params;

        let variables = params.variables.map_or(None, |variables| {
            Some(serde_json::from_str(&variables).unwrap())
        });

        Self::new(query, operation_name, variables)
    }
}

#[actix_web::get("/graphql")]
async fn graphql_get_handler(
    pool: Data<Pool>,
    schema: Data<Schema>,
    params: Query<GraphQLRequestSearchParams>,
) -> Result<HttpResponse> {
    let connection = database::connect(&pool)?;
    let context = Context::new(connection);

    let graphql_request = GraphQLRequest::from(params.into_inner());
    let graphql_response = graphql_request.execute(&schema, &context).await;
    let body = serde_json::to_string(&graphql_response)?;

    let mut response = if graphql_response.is_ok() {
        HttpResponse::Ok()
    } else {
        HttpResponse::BadRequest()
    };

    Ok(response.content_type("application/json").body(body))
}

#[actix_web::post("/graphql")]
async fn graphql_post_handler(
    pool: Data<Pool>,
    schema: Data<Schema>,
    payload: Payload,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let graphql_request = match request.content_type() {
        "application/graphql" => {
            let body = String::from_request(&request, &mut payload.into_inner()).await?;
            let graphql_request =
                GraphQLBatchRequest::Single(GraphQLRequest::new(body, None, None));

            Ok(graphql_request)
        }
        "application/json" => {
            let body = String::from_request(&request, &mut payload.into_inner()).await?;
            let graphql_request = serde_json::from_str(&body).map_err(ErrorBadRequest)?;

            Ok(graphql_request)
        }
        _ => Err(ErrorUnsupportedMediaType("")),
    }?;

    let connection = database::connect(&pool)?;
    let context = Context::new(connection);

    let graphql_response = graphql_request.execute(&schema, &context).await;
    let body = serde_json::to_string(&graphql_response)?;

    let mut response = if graphql_response.is_ok() {
        HttpResponse::Ok()
    } else {
        HttpResponse::BadRequest()
    };

    Ok(response.content_type("application/json").body(body))
}

#[actix_web::get("/graphql/explorer")]
async fn graphql_explorer_get_handler() -> Result<HttpResponse> {
    let body = playground::playground_source("/graphql", None);

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}
