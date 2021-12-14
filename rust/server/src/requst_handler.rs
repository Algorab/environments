use std::path::Path;
use std::sync::{LockResult, Mutex, MutexGuard, PoisonError};
use actix_web::{HttpRequest, HttpResponse, get, Responder};
use actix_web::guard::Guard;
use actix_web::http::StatusCode;
use libloading::{Library, Symbol};
use crate::{HandlerFunc, panic};
use crate::server::{CODE_PATH, load_plugin};


pub struct HandlerState {
    pub lib: Option<Library>,
    pub entry_point: String,
}

impl HandlerState {

    pub fn new() -> Self{
        Self {
            lib: None,
            entry_point: String::from("")
        }
    }

}

#[get("/")]
pub async fn user_handler(data: actix_web::web::Data<Mutex<HandlerState>>, req: HttpRequest) -> HttpResponse {

    let handler_state = data.lock().unwrap();

    match handler_state.lib.as_ref() {
        None => {
            //Todo: Return what fission here expect.
            error!("library not loaded");
            HttpResponse::BadRequest().body("not specialized")
        },
        Some(lib) => {

            let entry_point = &handler_state.entry_point;
            unsafe {
                let result = panic::catch_unwind_silent(||{
                    lib.get::<HandlerFunc>(&*entry_point.as_bytes()).unwrap()
                });

                match result {
                    Ok(symbol) => symbol(req),
                    //Todo: Return what fission here expect.
                    Err(e) => HttpResponse::InternalServerError().body(format!("handler:{} not available", *entry_point))
                }
            }
        }
    }

}
#[get("/healthz")]
pub async fn readiness_probe_handler() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/specialize")]
pub async fn specialize_handler<'handler_state>(data: actix_web::web::Data<Mutex<HandlerState>>) -> impl Responder {

    let handler_state= data.lock().unwrap();
    let mut user_func_lib = handler_state.lib.as_ref();

    match user_func_lib {
        Some(_) => {
            drop(handler_state);
            HttpResponse::BadRequest().body("Not a generic container")
        },
        None => {
            drop(handler_state);
            let path = Path::new(CODE_PATH);
            if !path.exists() {
                error!("code path ({}) does not exist", CODE_PATH);
                return HttpResponse::InternalServerError().body(format!("{} not found", CODE_PATH))
            }
            info!("specializing ...");
            load_plugin(&path, "handler", data);
            HttpResponse::Ok().body("Plugin Loaded")

        },

    }
}


// #[get("foo")]
// async fn foo(data: web::Data<HandlerState>, req: HttpRequest) -> HttpResponse {
//     let mut counter = data.user_func.lock().unwrap(); // <- get counter's MutexGuard
//
//     let func = counter.unwrap();
//
//     let resp = unsafe {
//         let resp = func(req);
//         resp
//     };
//
//     *counter = Some(|req|{
//         HttpResponse::Ok().body("fdsfsdlkfjdslafj")
//     }); // <- access counter inside MutexGuard
//
//     resp
//     //format!("Request number: {:?}", *resp.body().as_ref().unwrap()) // <- response with count
// }


