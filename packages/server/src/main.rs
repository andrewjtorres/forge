mod config;

use actix_web::{middleware::Logger, App, HttpResponse, HttpServer};
use dotenv;
use std::{env, io::Result};

use config::Config;

#[actix_web::get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "actix_web=info");

    dotenv::dotenv().ok();
    env_logger::init();

    let Config {
        host,
        port,
        ..
    } = Config::parse();

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(index)
    })
    .bind((host.as_str(), port))?
    .run();

    println!("The server is running at http://{}:{}", host, port);

    server.await
}
