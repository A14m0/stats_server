// defines config file releated functions
use chrono::prelude::*;


/// defines a partitular variable
pub struct Variable {
    name: String,
    value: String,
    date: chrono::DateTime<chrono::Local>
}

/// defines the structure of our config file for use later
pub struct Config {
    variables: Vec<Variable>
}

/// generates a new config file with `n` accesses
pub fn generate_new(n_accesses: u64, path: Option<String>) {
    // generate the JSON for the config

    // branch on output type
    match path {
        Some(a) => {
            // send output to file at path

        },
        None => {
            // print the output
        }
    }
}