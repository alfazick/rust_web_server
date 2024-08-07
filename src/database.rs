// src/database.rs

use crate::models::{Task,User};
use std::collections::HashMap;
use serde::{Serialize,Deserialize};
use std::io::Write;

#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Database {
    pub tasks: HashMap<u64, Task>,
    pub users: HashMap<u64, User>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            tasks: HashMap::new(),
            users: HashMap::new(),
        }
    }

    //CRUD DATA
    pub fn insert(&mut self, task: Task) {
        self.tasks.insert(task.id,task);
    }

    pub fn get(&self, id:&u64) -> Option<&Task> {
        self.tasks.get(id)
    }

    pub fn get_all(&self) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    pub fn delete(&mut self, id: &u64) {
        self.tasks.remove(id);
    }

    pub fn update(&mut self, task:Task){
        self.tasks.insert(task.id, task);
    }

    //Users

    pub fn insert_user(&mut self,user:User) {
        self.users.insert(user.id,user);
    }

    pub fn get_user_by_name(&self, username: &str) -> Option<&User> {
        self.users.values().find(|u| u.username == username)
    }

    // Data

    pub fn save_to_file(&self) -> std::io::Result<()> {
        let data = serde_json::to_string(&self)?;
        let mut file = std::fs::File::create("database.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn load_from_file() -> std::io::Result<Database> {
        let file_content = std::fs::read_to_string("database.json")?;
        let db: Database = serde_json::from_str(&file_content)?;
        Ok(db)
    }




}