use actix_web::{Responder, HttpResponse, get};

#[get("/health")]
pub async fn get_health() -> impl Responder {
    HttpResponse::Ok().body("Health is good!")
}
