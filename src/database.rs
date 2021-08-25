// defines functions and structs related to the internal database
use chrono::prelude::*;
use uuid::Uuid;
use serde_derive::{Serialize, Deserialize};
use std::io::Read;

// define our DatabaseVar class
#[derive(Serialize, Deserialize)]
pub struct DatabaseVar {
    name: String,
    value: String,
    date: String
}

// implement functions for DatabaseVar
impl DatabaseVar {
    /// creates a new one
    pub fn new() -> Self {
        DatabaseVar {
            name: String::new(),
            value: String::new(),
            date: String::new()
        }
    }
}

// define our Database structure
#[derive(Serialize, Deserialize)]
pub struct Database {
    vars: Vec<DatabaseVar>
}

// implement functions for Database
impl Database {
    /// create a clean Database instance 
    pub fn new() -> Self {
        Database{
            vars: Vec::new()
        }
    } 
}