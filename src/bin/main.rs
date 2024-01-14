use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../service/dbaccess/mod.rs"]
mod dbaccess;
#[path = "../service/errors.rs"]
mod errors;
#[path = "../service/handlers/mod.rs"]
mod handlers;
#[path = "../service/models/mod.rs"]
mod models;
#[path = "../service/routes.rs"]
mod routes;
#[path = "../service/state.rs"]
mod state;

use errors::EzyTutorError;
use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    // Construct App State
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    // Construct app and configure routes
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                EzyTutorError::InvalidInput("Please provide valid Json input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .configure(tutor_routes)
    };

    // Start HTTP server
    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");
    println!("Running on host:port = {:?}",host_port);
    HttpServer::new(app).bind(&host_port)?.run().await
}
