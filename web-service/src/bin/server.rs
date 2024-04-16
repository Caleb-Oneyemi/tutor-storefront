use actix_web::{web, App, HttpServer};
use slog::info;
use sqlx::postgres::PgPool;
use std::{env, io, sync::Mutex};

#[path = "../handlers/mod.rs"]
mod handlers;

#[path = "../models/mod.rs"]
mod models;

#[path = "../data/mod.rs"]
mod data;

#[path = "../state.rs"]
mod state;

#[path = "../routes.rs"]
mod routes;

#[path = "../errors.rs"]
mod errors;

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
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let conn_pool = PgPool::connect(&database_url).await.unwrap();
    let logger = get_logger();

    info!(logger, "Starting Server on {}...", addr);

    let shared_data = web::Data::new(AppState {
        visit_count: Mutex::new(0),
        db: conn_pool,
        logger: logger.clone(),
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(base_router)
            .configure(course_router)
            .configure(tutor_router)
    };

    let server = HttpServer::new(app).bind(addr)?.run().await;

    info!(logger, "Exiting Server...");

    server
}
