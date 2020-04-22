use actix_web::{
    error::ErrorInternalServerError,
    web::{Data, Json, ServiceConfig},
    Error, HttpResponse,
};
use juniper::http::{playground, GraphQLRequest};
use std::sync::Arc;

#[cfg(test)]
use crate::api::mocks::schema::{Context, Schema};
#[cfg(not(test))]
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
    request: Json<GraphQLRequest>,
) -> Result<HttpResponse, Error> {
    let response = request.execute(&schema, &Context {});
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

#[cfg(test)]
mod tests {
    use actix_web::{
        http::{header, Method, StatusCode},
        test, App,
    };
    use juniper::{self, http::GraphQLRequest, DefaultScalarValue};
    use std::sync::Arc;

    use super::{graphql_get_handler, graphql_post_handler};
    use crate::api;

    #[actix_rt::test]
    async fn graphql_get_handler_it_should_return_a_response_with_an_ok_status_code() {
        let server = test::start(|| App::new().service(graphql_get_handler));

        let response = server
            .request(Method::GET, server.url("/graphql"))
            .send()
            .await
            .unwrap();

        assert_eq!(StatusCode::OK, response.status());
        assert_eq!(
            "text/html; charset=utf-8",
            response.headers().get(header::CONTENT_TYPE).unwrap(),
        );
    }

    #[actix_rt::test]
    async fn graphql_post_handler_it_should_return_a_response_with_an_ok_status_code() {
        let request =
            GraphQLRequest::<DefaultScalarValue>::new("{ query }".to_string(), None, None);
        let schema = Arc::new(api::mocks::schema::create());
        let server = test::start(move || {
            App::new()
                .data(schema.clone())
                .service(graphql_post_handler)
        });

        let response = server
            .request(Method::POST, server.url("/graphql"))
            .send_json(&request)
            .await
            .unwrap();

        assert_eq!(StatusCode::OK, response.status());
        assert_eq!(
            "application/json",
            response.headers().get(header::CONTENT_TYPE).unwrap(),
        );
    }
}
