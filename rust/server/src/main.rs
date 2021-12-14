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
use server::{HandlerFunc};
use crate::requst_handler::{HandlerState, user_handler};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();

    #[allow(clippy::mutex_atomic)]
    let handler_state = web::Data::new(Mutex::new(HandlerState::new()));


    HttpServer::new(move || {
        App::new()
            .app_data(handler_state.clone())
            .service(requst_handler::readiness_probe_handler)
            .service(requst_handler::specialize_handler)
            .service(requst_handler::user_handler)
            //.route("/", web::get().to(manual_hello))
    })
        .bind("127.0.0.1:8888")?
        .run()
        .await
}

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//
//     let state = web::Data::new(HandlerState {
//         user_func: Mutex::new(Some(|req|{
//             HttpResponse::Ok().body("Hello")
//         })),
//     });
//
//     HttpServer::new(move || {
//         // move counter into the closure
//         App::new()
//             // Note: using app_data instead of data
//             .app_data(state.clone()) // <- register the created data
//             .service(user_handler)
//     })
//         .bind("127.0.0.1:8888")?
//         .run()
//         .await
// }