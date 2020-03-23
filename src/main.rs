use actix_web::{web, App, Responder, HttpServer, HttpResponse};
// autoreloading
use listenfd::ListenFd;
use crate::db_connection::establish_connection;

pub mod handlers; // This goes to the top to load the next handlers module
pub mod utils;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

pub mod schema;
pub mod models;
pub mod db_connection;
pub mod errors;

#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate actix_web;
extern crate futures;


// systemfd --no-pid -s http::3000 -- cargo watch -x run
const IP: &str = "127.0.0.1:3000";


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server =
        HttpServer::new(|| {
            App::new()
                .data(establish_connection())
                .service(
                    web::resource("/products")
                        .route(web::get().to(handlers::products::index))
                        .route(web::post().to(handlers::products::create))
                )

                .service(
                    web::resource("/products/{id}")
                        .route(web::get().to(handlers::products::show))
                        .route(web::delete().to(handlers::products::destroy))
                        .route(web::patch().to(handlers::products::update))
                )

        });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        println!("Started http server: {}", IP);
        server.bind(&IP)?
    };

    server.run().await
}

