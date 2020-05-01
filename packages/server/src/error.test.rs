use actix_web::{
    body::Body,
    error::ResponseError,
    http::{header, StatusCode},
    web::Bytes,
};

use super::{Error, UNABLE_TO_CONNECT_TO_DATABASE};

#[test]
fn error_it_should_implement_debug() {
    assert_eq!(
        UNABLE_TO_CONNECT_TO_DATABASE,
        format!("{:?}", Error::UnableToConnectToDatabase)
    );
}

#[test]
fn error_it_should_implement_display() {
    assert_eq!(
        UNABLE_TO_CONNECT_TO_DATABASE,
        format!("{}", Error::UnableToConnectToDatabase)
    );
}

#[test]
fn error_it_should_implement_response_error() {
    let response = Error::UnableToConnectToDatabase.error_response();

    assert_eq!(StatusCode::INTERNAL_SERVER_ERROR, response.status());
    assert_eq!(
        "text/plain; charset=utf-8",
        response.headers().get(header::CONTENT_TYPE).unwrap()
    );
    assert_eq!(
        Body::Bytes(Bytes::from(UNABLE_TO_CONNECT_TO_DATABASE)),
        *response.body().as_ref().unwrap()
    );
}
