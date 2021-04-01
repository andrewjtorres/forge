mod graphql;

use actix_web::web::ServiceConfig;

use graphql::{graphql_explorer_get_handler, graphql_get_handler, graphql_post_handler};

pub fn configure(config: &mut ServiceConfig) {
    config
        .service(graphql_get_handler)
        .service(graphql_post_handler);

    #[cfg(debug_assertions)]
    config.service(graphql_explorer_get_handler);
}
