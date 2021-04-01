#[macro_use]
extern crate diesel;

mod api;
mod config;
mod database;
mod error;
mod models;
mod routes;

use actix_cors::Cors;
use actix_web::{
    http::{header, Method},
    middleware::{Compress, Logger},
    web::Data,
    App, HttpServer,
};
use std::{env, io::Result, sync::Arc};

use api::schema::Schema;
use config::Config;

#[actix_web::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");

    dotenv::dotenv().ok();
    env_logger::init();

    let Config {
        database_url,
        host,
        port,
        ..
    } = Config::parse();
    let origin = format!("http://{}:{}", host, port);
    let pool = database::pool::connect(database_url.as_str());
    let schema = Arc::new(api::schema::create());

    let server = HttpServer::new(move || {
        App::new()
            .app_data::<Data<Schema>>(schema.clone().into())
            .data(pool.clone())
            .wrap(Compress::default())
            .wrap(
                Cors::default()
                    .allowed_headers(vec![
                        header::ACCEPT,
                        header::AUTHORIZATION,
                        header::CONTENT_TYPE,
                    ])
                    .allowed_methods(vec![Method::GET, Method::POST])
                    .allowed_origin(origin.clone().as_str())
                    .max_age(3600)
                    .supports_credentials(),
            )
            .wrap(Logger::default())
            .configure(routes::configure)
    })
    .bind((host.as_str(), port))?
    .run();

    println!("The server is running at http://{}:{}", host, port);

    server.await
}
