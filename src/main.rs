extern crate log;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;

mod health_handler;
mod events_handler;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // Environment variables
    dotenv::dotenv().ok();
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    // Start http server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i %D"))
            .service(health_handler::get_health)
            .service(events_handler::get_events)
            .service(events_handler::get_event_by_id)
            .service(events_handler::create_event)
            .service(events_handler::update_event)
            .service(events_handler::delete_event)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

}

