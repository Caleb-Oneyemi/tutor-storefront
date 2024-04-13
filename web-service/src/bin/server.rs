use actix_web::{web, App, HttpServer};
use std::{io, sync::Mutex};

#[path = "../state.rs"]
mod state;

#[path = "../handlers.rs"]
mod handlers;

#[path = "../routes.rs"]
mod routes;

use routes::router;
use state::AppState;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let shared_data = web::Data::new(AppState {
        health_check_response: "server is up and running".to_string(),
        visit_count: Mutex::new(0),
    });

    let app = move || App::new().app_data(shared_data.clone()).configure(router);

    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
