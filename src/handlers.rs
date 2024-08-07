// src/handlers.rs

use actix_web::{web, HttpResponse, Responder};
use crate::AppState;
use crate::database::Database;
use crate::models::{Task,User};


use std::sync::MutexGuard;

pub async  fn read_all_tasks(app_state: web::Data<AppState>) -> impl Responder {
    let db: MutexGuard<Database> = app_state.db.lock().unwrap();
    let tasks = db.get_all();
    HttpResponse::Ok().json(tasks)
}

pub async fn create_task(app_state:web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db: MutexGuard<Database> = app_state.db.lock().unwrap();
    db.update(task.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

pub async fn update_task(app_state:web::Data<AppState>, task: web::Json<Task>) -> impl Responder {
    let mut db: MutexGuard<Database> = app_state.db.lock().unwrap();
    db.update(task.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

pub async fn delete_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let mut db: MutexGuard<Database> = app_state.db.lock().unwrap();
    db.delete(&id.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

pub async fn read_task(app_state: web::Data<AppState>, id: web::Path<u64>) -> impl Responder {
    let db: MutexGuard<Database> = app_state.db.lock().unwrap();
    db.get(&id.into_inner());
    HttpResponse::Ok().finish()
}

pub async fn register(app_state: web::Data<AppState>, user:web::Json<User>) -> impl Responder {
    let mut db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    db.insert_user(user.into_inner());
    let _ = db.save_to_file();
    HttpResponse::Ok().finish()
}

pub async fn login(app_state: web::Data<AppState>, user: web::Json<User>) -> impl Responder {
    let db: std::sync::MutexGuard<Database> = app_state.db.lock().unwrap();
    match db.get_user_by_name(&user.username) {
        Some(stored_user) if stored_user.password == user.password => {
            HttpResponse::Ok().body("Logged in!")
        },
        _ => HttpResponse::BadRequest().body("Invalid username or password")
    }
}





