use std::ffi::OsString;
use std::fs;
use std::fs::ReadDir;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard};

use actix_web::{HttpRequest, HttpResponse};
use actix_web::web::Data;
use libloading::{Library};
use log::info;

use crate::{HandlerState};

pub type HandlerFunc = unsafe fn(HttpRequest) -> HttpResponse;

pub const CODE_PATH: &str = "/userfunc/user";
//pub const CODE_PATH: &str = "/home/stefan/workspace/kubernetes/fission-rust-handler/target/debug/lib";


pub fn specializer(data: Data<Mutex<HandlerState>>, code_path: &str, handler_name: &str) -> HttpResponse {
    let handler_state = data.lock().unwrap();
    let user_func_lib = handler_state.lib.as_ref();

    match user_func_lib {
        Some(_) => {
            drop(handler_state);
            HttpResponse::BadRequest().body("Not a generic container")
        },
        None => {
            drop(handler_state);
            let path = Path::new(code_path);
            if !path.exists() {
                error!("code path ({}) does not exist", code_path);
                HttpResponse::NotFound().body(format!("{} not found", code_path))
            } else {
                info!("specializing ...");
                match load_plugin(&path, handler_name, data) {
                    Ok(_) => HttpResponse::Ok().body("Plugin Loaded"),
                    Err(e) => HttpResponse::InternalServerError().body(e)
                }
            }
        },
    }
}

fn load_plugin(code_path: &Path, entry_point: &str, data: actix_web::web::Data<Mutex<HandlerState>>) -> Result<(), String> {
    if code_path.is_dir() {
        //Todo: 2. use a reference for OsString
        match fs::read_dir(code_path) {
            Err(e) => {
                Err(format!("error reading directory: {}", e))
            },
            Ok(read_dir) => {
                info!("reading directory: {}", code_path.to_str().unwrap());
                match read_directory(read_dir) {
                    None => Err(
                        format!(
                            "No file found to load in directory: {}",
                            code_path.to_str().unwrap()
                        )
                    ),
                    Some(file_name) => {
                        let lib_path = build_library_path(code_path, file_name);
                        let mut handler_state = data.lock().unwrap();
                        match load_library(entry_point, lib_path, &mut handler_state) {
                            Err(e) => {
                                drop(handler_state);
                                Err(e)
                            },
                            Ok(()) => {
                                drop(handler_state);
                                Ok(())
                            }
                        }
                    }
                }
            }
        }
    } else {
        Err(format!("error checking plugin path: {}", code_path.to_str().unwrap()))
    }
}

fn load_library(entry_point: &str, lib_path: PathBuf, handler_state: &mut MutexGuard<HandlerState>) -> Result<(), String> {
    info!("Loading  plugin from {}", lib_path.to_str().unwrap());
    unsafe {
        match Library::new(&lib_path) {
            Ok(lib) => {
                (*handler_state).entry_point = entry_point.to_string();
                (*handler_state).lib = Some(lib);
                Ok(())
            }
            Err(e) => {
                Err(format!("Lib {} can't be load error: {}", &lib_path.to_str().unwrap(), e.to_string()))
            }
        }
    }
}

fn build_library_path(code_path: &Path, file_name: OsString) -> PathBuf {
    let full_path = code_path.join(file_name);
    info!("library to load in directory: {}", full_path.to_str().unwrap());
    full_path
}

fn read_directory(read_dir: ReadDir) -> Option<OsString> {
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
}