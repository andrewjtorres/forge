use actix_web::{
    http::{header, Method, StatusCode},
    test, App,
};

use super::graphql_explorer_get_handler;

#[actix_rt::test]
async fn graphql_explorer_get_handler_it_should_return_a_response_with_an_ok_status_code() {
    let server = test::start(|| App::new().service(graphql_explorer_get_handler));

    let mut response = server
        .request(Method::GET, server.url("/graphql/explorer"))
        .send()
        .await
        .unwrap();

    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(
        "text/html; charset=utf-8",
        response.headers().get(header::CONTENT_TYPE).unwrap(),
    );
    assert!(!response.body().await.unwrap().is_empty())
}
