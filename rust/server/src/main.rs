mod server;
mod requst_handler;
mod panic;

#[macro_use] extern crate serde;

#[macro_use]
extern crate log;

use std::sync::Mutex;
use actix_web::{App, HttpServer, web};
use server::{HandlerFunc};
use crate::requst_handler::{HandlerState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=trace");
    env_logger::init();

    #[allow(clippy::mutex_atomic)]
    let handler_state = web::Data::new(Mutex::new(HandlerState::new()));


    HttpServer::new(move || {
        App::new()
            .app_data(handler_state.clone())
            .service(requst_handler::readiness_probe_handler)
            .service(requst_handler::specialize_handler)
            .service(requst_handler::specialize_handler_v2)
            .service(requst_handler::user_handler)
    })
        .bind("0.0.0.0:8888")?
        .run()
        .await
}