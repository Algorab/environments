use std::sync::Mutex;
use actix_web::{HttpRequest, HttpResponse, get, Responder, web};
use libloading::{Library};
use crate::{HandlerFunc, panic, server};
use crate::server::{CODE_PATH};

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

#[derive(Deserialize, Debug)]
pub struct FunctionLoadRequest {
    #[serde(alias = "filepath")]
    pub file_path: String,

    #[serde(alias = "functionName")]
    pub function_name: String,

    //currently not use. I see this in the server.go from in the go env
    pub url: String,
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
                    Ok(symbol) => *symbol(Box::new(req)),
                    Err(_) => HttpResponse::InternalServerError().body(format!("handler:{} not available", *entry_point))
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
pub async fn specialize_handler(data: actix_web::web::Data<Mutex<HandlerState>>) -> impl Responder {
    server::specializer(data, CODE_PATH, "handler")
}

#[get("/v2/specialize")]
pub async fn specialize_handler_v2(data: actix_web::web::Data<Mutex<HandlerState>>, function_load_request: web::Json<FunctionLoadRequest>) -> impl Responder {
    server::specializer(data, function_load_request.file_path.as_str(), function_load_request.function_name.as_str())
}

