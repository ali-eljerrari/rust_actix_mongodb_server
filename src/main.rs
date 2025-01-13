// Main application module for the REST API server.
//
// This module sets up and runs the HTTP server with MongoDB integration.
// It handles configuration, database connection, routing, and server startup.

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, middleware::Logger};
use mongodb::{Client, Database};
use mongodb::options::ClientOptions;
use mongodb::bson::doc;
use env_logger;
use std::env;
use dotenv::dotenv;

mod models;
mod routes;
mod services;
mod controllers;

/// Application state shared across request handlers
pub struct AppState {
    /// MongoDB database connection
    db: Database,
}

/// Health check endpoint that returns server status
#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now()
    }))
}

/// Root endpoint that returns API information
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Welcome to the API",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

/// Main function that initializes and runs the HTTP server
///
/// # Steps:
/// 1. Loads environment variables
/// 2. Initializes logging
/// 3. Connects to MongoDB
/// 4. Sets up application state
/// 5. Configures and starts the HTTP server with all routes
///
/// # Errors
/// Returns `std::io::Error` if server fails to start
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Get MongoDB connection details from environment
    let uri = env::var("MONGO_URI").expect("MONGO_URI not set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME not set");

    // Configure and establish MongoDB connection
    let mut client_options = ClientOptions::parse(uri)
        .await
        .expect("Failed to parse MongoDB connection string");
    
    client_options.app_name = Some("Rust App".to_string());

    let client = Client::with_options(client_options)
        .expect("Failed to initialize MongoDB client");
    
    // Verify MongoDB connection
    client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await
        .expect("Failed to ping MongoDB");

    println!("Successfully connected to MongoDB!");

    let database = client.database(&database_name);
    
    // Initialize shared application state
    let app_state = web::Data::new(AppState { db: database });

    println!("Starting server at http://127.0.0.1:8080");

    // Configure and start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .service(health_check)
            .service(index)
            .service(routes::user_route::get_users)
            .service(routes::user_route::get_user_by_id)
            .service(routes::user_route::create_user)
            .service(routes::user_route::update_user)
            .service(routes::user_route::delete_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}