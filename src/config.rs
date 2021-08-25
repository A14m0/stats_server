// defines config file releated functions
use serde_derive::{Serialize, Deserialize};
use std::io::Read;

/// defines a partitular variable
#[derive(Serialize, Deserialize)]
pub struct Variable {
    name: String,
    value: String,
    date: String
}

/// defines the structure of our config file for use later
#[derive(Serialize, Deserialize)]
pub struct Config {
    server_name: String,
    allowed_users: Vec<String>,
    variables: Vec<String>,
    port: u16
}

// implement functions for the variables
impl Variable {

}

// implement functions for the config file
impl Config {
    /// exposes the port
    pub fn port(&self) -> u16 {
        self.port
    }

    /// exposes the server's name
    pub fn server_name(&self) -> String {
        self.server_name.clone()
    }

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

/// opens an existing config file from path
pub fn open_config(path: String) -> Result<Config, String> {
    let conf_dat: Config;
    // attempt to open the config file
    match std::fs::File::open(path){
        Ok(mut file) => {
            // try to parse it
            let mut data = String::new();
            file.read_to_string(&mut data).unwrap();
            conf_dat = serde_json::from_str(&data[..]).unwrap();

        },
        Err(e) => {
            return Err(format!("Illicit config file: {}", e));
        }
    };
    //let mut uuids: Vec<String> = Vec::new();
    //let mut variables: Vec<Variable> = Vec::new();
    //let port = match conf_dat.port {
    //    true => 443,
    //    false => match conf_dat["port"] {
    //        json::JsonValue::Number(n) => n,
    //        _ => panic!("Illicit data type for port!")
    //    }
    //};

    Ok(conf_dat)
}
