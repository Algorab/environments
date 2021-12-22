use actix_web::{HttpRequest, HttpResponse};

#[no_mangle]
pub extern fn handler(_req: Box<HttpRequest>) -> Box<HttpResponse> {
    Box::new(HttpResponse::Ok().body("Hello from dynamic loaded lib :-)"))
}

