#[cfg(test)]
#[path = "error.test.rs"]
mod tests;

use actix_web::{error::ResponseError, HttpResponse};
use std::{
    fmt::{self, Debug, Display, Formatter},
    result,
};

pub type Result<T, E = Error> = result::Result<T, E>;

const UNABLE_TO_CONNECT_TO_DATABASE: &str = "Unable to establish a database connection";

pub enum Error {
    UnableToConnectToDatabase,
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Error::UnableToConnectToDatabase => write!(f, "{}", UNABLE_TO_CONNECT_TO_DATABASE),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Error::UnableToConnectToDatabase => write!(f, "{}", UNABLE_TO_CONNECT_TO_DATABASE),
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        match *self {
            Error::UnableToConnectToDatabase => HttpResponse::InternalServerError()
                .content_type("text/plain; charset=utf-8")
                .body(UNABLE_TO_CONNECT_TO_DATABASE),
        }
    }
}
