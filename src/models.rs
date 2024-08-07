// src/models.rs
use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Task {
    pub id: u64,
    pub name: String,
    pub completed: bool,
}

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct User {
    pub id:u64,
    pub username: String,
    pub password: String,
}


