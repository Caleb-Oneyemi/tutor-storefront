use actix_web::{web, App, HttpServer};
use slog::info;
use std::{io, sync::Mutex};

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

use logger::get_logger;
use routes::router;
use state::AppState;

#[actix_web::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    let logger = get_logger();

    info!(logger, "Starting Server...");

    let shared_data = web::Data::new(AppState {
        visit_count: Mutex::new(0),
        courses: Mutex::new(Vec::new()),
        logger: logger.clone(),
    });

    let app = move || App::new().app_data(shared_data.clone()).configure(router);

    let server = HttpServer::new(app).bind("127.0.0.1:3000")?.run().await;

    info!(logger, "Exiting Server...");

    server
}
