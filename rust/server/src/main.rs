mod server;
mod requst_handler;
mod panic;

#[macro_use] extern crate serde;

#[macro_use]
extern crate log;


use std::fmt::Pointer;
use std::ops::Add;
use std::sync::Mutex;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web, get};
use actix_web::body::Body;
use env_logger::{Env, Logger};
use server::{HandlerFunc};
use crate::requst_handler::{HandlerState, user_handler};

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
        .bind("127.0.0.1:8888")?
        .run()
        .await
}