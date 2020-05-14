use actix_web::Responder;
use actix_web::{web, HttpResponse, get, post, put, delete};
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

// Structs
#[derive(Debug, Serialize, Deserialize)]
pub struct InputEvent {
    pub name: String,
    pub start_date: chrono::DateTime<Utc>,
    pub end_date: chrono::DateTime<Utc>,
    pub location: String,
    pub paid_entrance: bool,
    pub price: f32
}

// Public functions
#[get("/events")]
pub async fn get_events() -> impl Responder {
    HttpResponse::Ok().body("Get events!")
}

#[get("/events/{id}")]
pub async fn get_event_by_id(id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Get event: {}", id))
}

#[post("/events")]
pub async fn create_event(item: web::Json<InputEvent>) -> impl Responder {
    HttpResponse::Ok().body(format!("Create event: {}", item.name))
}

#[put("/events/{id}")]
pub async fn update_event(id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Update event: {}", id))
}

#[delete("/events/{id}")]
pub async fn delete_event(id: web::Path<i32>) -> impl Responder {
    HttpResponse::Ok().body(format!("Delete event: {}", id))
}


// Private functions