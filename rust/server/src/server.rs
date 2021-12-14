use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Mutex, MutexGuard};

use actix_web::{get, HttpRequest, HttpResponse, Responder, web};
use actix_web::http::StatusCode;
use libloading::{Library, Symbol};
use log::{info};

use crate::HandlerState;

pub type HandlerFunc = unsafe fn(HttpRequest) -> HttpResponse;

//Todo: put FunctionLoadRequest to the request_handlers
#[derive(Deserialize, Debug)]
pub struct FunctionLoadRequest {
    #[serde(alias = "filepath")]
    pub file_path: String,

    #[serde(alias = "functionName")]
    pub function_name: String,

    //currently not use. I see this in the server.go from in the go env
    pub url: String,
}

//const CODE_PATH: &str = "/userfunc/user";
pub const CODE_PATH: &str = "/home/stefan/workspace/kubernetes/fission-rust-handler/target/debug/lib";


//Todo: Return http response for error cases or done
pub fn load_plugin(code_path: &Path, entry_point: &str, data: actix_web::web::Data<Mutex<HandlerState>>) {
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

        let mut handler_state = data.lock().unwrap();

        match lib_path {
            Some(plugin) => {
                info!("Loading  plugin from {}", plugin.to_str().unwrap());
                unsafe {
                    let lib = Library::new(plugin).unwrap();

                    (*handler_state).entry_point = entry_point.to_string();
                    (*handler_state).lib = Some(lib);
                }
            },
            None => {
                error!("no library to load found");
            }
        }
        drop(handler_state);
    } else {
        error!("error checking plugin path: {}", code_path.to_str().unwrap());
    }
}