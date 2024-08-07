


// src/main.rs
mod models;
mod database;
mod handlers;

use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use crate::database::Database;
use actix_cors::Cors;
use actix_web::http::header;


struct AppState {
    db: Mutex<Database>,
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db: Database = match Database::load_from_file() {
        Ok(db) => db,
        Err(_) => Database::new()
    };

    let data: web::Data<AppState> = web::Data::new(AppState { 
        db: Mutex::new(db) 
    });

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::permissive()
                    .allowed_origin_fn(|origin, _req_head| {
                        origin.as_bytes().starts_with(b"http://localhost") || origin == "null"
                    })
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600)
            )
            .app_data(data.clone())
            .route("/task", web::post().to(handlers::create_task))
            .route("/task", web::get().to(handlers::read_all_tasks))
            .route("/task", web::put().to(handlers::update_task))
            .route("/task/{id}", web::get().to(handlers::read_task))
            .route("/task/{id}", web::delete().to(handlers::delete_task))
            .route("/register", web::post().to(handlers::register))
            .route("/login", web::post().to(handlers::login))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

