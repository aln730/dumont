use actix_web::{get, HttpResponse};

#[get("/printer_status")]
pub async fn printer_status() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}
