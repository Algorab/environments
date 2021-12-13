use std::path::Path;
use std::sync::{Mutex, MutexGuard};
use actix_web::{HttpRequest, HttpResponse, get, Responder};
use actix_web::http::StatusCode;
use libloading::{Library, Symbol};
use crate::{HandlerFunc, panic};
use crate::server::{CODE_PATH, load_plugin};


pub struct HandlerState {
    pub lib: Mutex<Option<Library>>,
    pub entry_point: Mutex<String>,
}

impl HandlerState {

    pub fn new() -> Self{
        Self {
            lib: Mutex::new(None),
            entry_point: Mutex::new(String::from(""))
        }
    }

}

#[get("/")]
pub async fn user_handler(data: actix_web::web::Data<HandlerState>, req: HttpRequest) -> HttpResponse {

    let lib = data.lib.lock().unwrap(); // <- get counter's MutexGuard
    let entry_point = data.entry_point.lock().unwrap();

    unsafe {

        println!("entrypoint: {}", entry_point);
        let result = panic::catch_unwind_silent(||{
            let o = lib.as_ref().unwrap();
            o.get::<HandlerFunc>(&*entry_point.as_bytes()).unwrap()
        });

        match result {
            Ok(symbol) => symbol(req),
            //Todo: Return what fission here expect.
            Err(e) => HttpResponse::InternalServerError().body(format!("handler:{} not available", *entry_point))
        }
    }

}
#[get("/healthz")]
pub async fn readiness_probe_handler() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/specialize")]
pub async fn specialize_handler(handler_state: actix_web::web::Data<HandlerState>) -> impl Responder {

    let mut user_func_lib: MutexGuard<Option<Library>> = handler_state.lib.lock().unwrap();

    match *user_func_lib {
        Some(_) => {
            drop(user_func_lib);
            HttpResponse::BadRequest().body("Not a generic container")
        },
        None => {
            drop(user_func_lib);
            let path = Path::new(CODE_PATH);
            if !path.exists() {
                error!("code path ({}) does not exist", CODE_PATH);
                return HttpResponse::InternalServerError().body(format!("{} not found", CODE_PATH))
            }
            info!("specializing ...");
            load_plugin(&path, "handler", &handler_state);
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


