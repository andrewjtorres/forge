mod graphql;

use actix_web::web::ServiceConfig;

pub fn configure(config: &mut ServiceConfig) {
    graphql::configure(config);
}
