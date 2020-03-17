mod api;
mod config;
mod database;
mod routes;

use actix_web::{middleware::Logger, App, HttpServer};
use dotenv;
use std::{env, io::Result, sync::Arc};

use config::Config;
use database::pool;

#[actix_rt::main]
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
    let schema = Arc::new(api::schema::create());

    let server = HttpServer::new(move || {
        App::new()
            .data(pool::connect(database_url.as_str()))
            .data(schema.clone())
            .wrap(Logger::default())
            .configure(routes::configure)
    })
    .bind((host.as_str(), port))?
    .run();

    println!("The server is running at http://{}:{}", host, port);

    server.await
}
