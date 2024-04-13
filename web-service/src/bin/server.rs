use actix_web::{web, App, HttpServer};
use slog::info;
use std::{env, io, sync::Mutex};

#[path = "../state.rs"]
mod state;

#[path = "../handlers.rs"]
mod handlers;

#[path = "../routes.rs"]
mod routes;

#[path = "../models.rs"]
mod models;

#[path = "../logger.rs"]
mod logger;

use dotenv::dotenv;
use logger::get_logger;
use routes::*;
use state::AppState;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let addr = env::var("API_URL").expect("API_URL must be set");

    let logger = get_logger();

    info!(logger, "Starting Server on {}...", addr);

    let shared_data = web::Data::new(AppState {
        visit_count: Mutex::new(0),
        courses: Mutex::new(Vec::new()),
        logger: logger.clone(),
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(base_router)
            .configure(course_router)
    };

    let server = HttpServer::new(app).bind(addr)?.run().await;

    info!(logger, "Exiting Server...");

    server
}
