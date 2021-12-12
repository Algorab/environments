use std::path::Path;
use std::sync::Mutex;
use actix_web::{HttpRequest, HttpResponse, web, get, Responder};
use actix_web::http::StatusCode;
use crate::HandlerFunc;
use crate::server::{CODE_PATH, load_plugin};


pub struct HandlerState {
    pub user_func: Mutex<Option<HandlerFunc>>,
}


impl HandlerState {
    // pub fn new() -> Self{
    //     Self {
    //         user_func: Mutex::new(Some(|req|{
    //             HttpResponse::Ok().body("Hello")
    //         })),
    //     }
    // }

    pub fn new() -> Self{
        Self {
            user_func: Mutex::new(None),
        }
    }


}

#[get("/")]
pub async fn user_handler(data: web::Data<HandlerState>, req: HttpRequest) -> HttpResponse {

    let mut handler = data.user_func.lock().unwrap(); // <- get counter's MutexGuard

    let func = handler.unwrap();

    // let resp = unsafe {
    //     let resp = func(req);
    //     resp
    // };

    let resp = HttpResponse::Ok().body("Hello");

    *handler = Some(|req|{
        HttpResponse::Ok().body("fdsfsdlkfjdslafj")
    }); // <- access counter inside MutexGuard

    resp
}
#[get("/healthz")]
pub async fn readiness_probe_handler() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/specialize")]
pub async fn specialize_handler(handler_state: web::Data<HandlerState>, req: HttpRequest) -> impl Responder {
    let mut app_user_func = handler_state.user_func.lock().unwrap();

    match *app_user_func {
        Some(_) => HttpResponse::BadRequest().body("Not a generic container"),
        None => {
            let path = Path::new(CODE_PATH);
            if !path.exists() {
                error!("code path ({}) does not exist", CODE_PATH);
                return HttpResponse::InternalServerError().body(format!("{} not found", CODE_PATH))
            }
            info!("specializing ...");
            let loaded_user_func = load_plugin(&path, "handler", req);
            match loaded_user_func {
                Some(func) => {
                    *app_user_func = Some(func);
                    HttpResponse::Ok().body("Plugin Loaded")


                },
                None => {
                    error!("No user_func found");
                    HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }


        },

    }
}


#[get("foo")]
async fn foo(data: web::Data<HandlerState>, req: HttpRequest) -> HttpResponse {
    let mut counter = data.user_func.lock().unwrap(); // <- get counter's MutexGuard

    let func = counter.unwrap();

    let resp = unsafe {
        let resp = func(req);
        resp
    };

    *counter = Some(|req|{
        HttpResponse::Ok().body("fdsfsdlkfjdslafj")
    }); // <- access counter inside MutexGuard

    resp
    //format!("Request number: {:?}", *resp.body().as_ref().unwrap()) // <- response with count
}


