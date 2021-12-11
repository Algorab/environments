mod server_handler;

#[macro_use] extern crate serde;

use actix_web::{App, HttpServer};
use crate::server_handler::HandlerState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .data(HandlerState::new())
            .service(server_handler::readiness_probe_handler)
            .service(server_handler::specialize_handler)
            .service(server_handler::user_handler)
            //.route("/", web::get().to(manual_hello))
    })
        .bind("127.0.0.1:8888")?
        .run()
        .await
}
