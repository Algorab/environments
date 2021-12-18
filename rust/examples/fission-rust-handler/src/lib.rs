use actix_web::{HttpRequest, HttpResponse};

#[no_mangle]
pub extern "C" fn handler(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().body("Hello from dynamic loaded lib :-)")
}

