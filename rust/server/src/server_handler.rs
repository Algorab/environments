use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use actix_web::http::StatusCode;
use libloading::{Library, Symbol};

use log::{error, info};

pub struct HandlerState {
    user_func: Mutex<Option<HandlerFunc>>,
}

impl HandlerState {
    pub fn new() -> Self{
        Self {
            user_func: Mutex::new(None),
        }
    }
}

pub type HandlerFunc = unsafe fn(HttpRequest) -> HttpResponse;

#[derive(Serialize, Deserialize)]
struct FunctionLoadRequest {
    #[serde(alias="filepath")]
    file_path: String,

    #[serde(alias="functionName")]
    function_name: String,

    url: String
}

const CODE_PATH: &str = "/userfunc/user";

#[get("/healthz")]
pub async fn readiness_probe_handler() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/specialize")]
pub async fn specialize_handler(handler_state: web::Data<HandlerState>) -> impl Responder {
    let user_func = *handler_state.user_func.lock().unwrap();

    match user_func {
        Some(_) => HttpResponse::BadRequest().body("Not a generic container"),
        None => {
            let path = Path::new(CODE_PATH);
            if !path.exists() {
                error!("code path ({}) does not exist", CODE_PATH);
                return HttpResponse::InternalServerError().body(format!("{} not found", CODE_PATH))
            }
            info!("specializing ...");
            let user_func = load_plugin(&path, "Handler");
            match user_func {
                Some(func) => {
                    *handler_state.user_func.lock().unwrap() = Some(func);
                    HttpResponse::new(StatusCode::OK)
                },
                None => {
                    error!("No user_func found");
                    HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }


        },

    }
}

#[get("/")]
pub async fn user_handler() -> HttpResponse {
    HttpResponse::Ok().body("user_handler")
    //HttpResponse::new(StatusCode::OK)

}


fn load_plugin(code_path: &Path, entry_point:&str) -> Option<HandlerFunc>{
    if code_path.is_dir() {
        //Todo: 1. swtich from Option to Result
        //Todo: 2. use a reference for OsString
        let file_name: Option<OsString> = match fs::read_dir(code_path) {
            Err(e) => {
                error!("error reading directory: {}", e);
                None
            },
            Ok(read_dir) => {
                info!("reading directory: {}", code_path.to_str().unwrap());
                read_dir.map(|dir_entry| {

                    let entry = dir_entry.unwrap();
                    let file_type = entry.file_type().unwrap();
                    if file_type.is_file() {
                        Some(entry.file_name())
                    } else {
                        None
                    }
                }).filter(|e| e.is_some())
                    .map(|some| some.unwrap())
                    .collect::<Vec<OsString>>().first().cloned()
            },
        };

       let lib_path = match file_name {
           Some(name) => {
               let full_path = code_path.join(name);
               info!("library to load in directory: {}", full_path.to_str().unwrap());
               Some(code_path.join(full_path))
           },
           None => {
               error!("no library to load in directory: {}", CODE_PATH);
               None
           }
       };

       match lib_path {
           Some(plugin) => {
               info!("Loading  plugin from {}", plugin.to_str().unwrap());
               unsafe {
                   let lib = Library::new(plugin).unwrap();
                   let symbol: Symbol<HandlerFunc> = lib.get(entry_point.as_bytes()).unwrap();
                   Some(*symbol)
               }
           },
           None => {
               None
           }
       }

    } else {
        error!("error checking plugin path: {}", code_path.to_str().unwrap());
        None
    }
}