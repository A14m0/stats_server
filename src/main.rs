use warp::Filter;
use clap::{Arg, App, SubCommand};
use std::sync::Arc;
use futures::lock::Mutex;
use std::convert::Infallible;

mod handler;
mod config;
mod log;
mod database;
use log::{
    log,
    LTYPE
};

async fn run_server(config_struct: config::Config, dbpath: Option<String>) {//db: Arc<Mutex<database::Database>>) {
    log(LTYPE::Info, format!("Started web server"));

    // create a new mutex for a handler object
    let db = match database::init_db(dbpath) {
            Ok(a) => a,
            Err(e) => panic!("Failed to open database! ({})", e)
        };

    // favicon path filter
    //let favico = warp::path("favicon.ico").and(warp::fs::file("favicon.ico"));

    // api filters
    let api_root = warp::path!("api"/..);
    
    let push = warp::path!("post" / String / String)
        .and(warp::post())
        .and(with_db(db.clone()))
        .and_then( move |uuid, encdat, db1: database::Db|  {
                    handler::post_handle(uuid, encdat, db1)
            }
        );
    let get = warp::path!("get" / String / String)
        .and(with_db(db))
        .and_then(move |uuid, encdat, db2| {
                    handler::get_handle(uuid, encdat, db2)        
            } 
            
        );

    let api = api_root
        .and(push.or(get));
    //let routes = favico
    //    .or(hi);
    // run the server, and point it to the keys
    warp::serve(api)
        .tls()
        .cert_path("./cert.pem")
        .key_path("./key.pem")
        .run(([127,0,0,1],config_struct.port()))
        .await;
}

fn with_db(db: database::Db) -> impl Filter<Extract = (database::Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

#[tokio::main]
async fn main() {
    // parse the CLI arguments
    let matches =  App::new("Stats Server")
                        .version("0.1")
                        .about("Serves up statistics")
                        .subcommand(SubCommand::with_name("new")
                            .about("Creates a new config file")
                            .version("1.0")
                            .arg(Arg::with_name("file")
                                .short("f")
                                .long("file")
                                .help("Saves config to a file")
                                .value_name("FILE")
                                .takes_value(true)
                            )
                            .arg(Arg::with_name("number")
                                .short("n")
                                .long("number")
                                .help("The number of access entries for the config file")
                                .value_name("NUM")
                                .takes_value(true)
                                .required(true)
                            )
                        )
                        .subcommand(SubCommand::with_name("run")
                            .about("Run the server")
                            .version("0.1")
                            .arg(Arg::with_name("config")
                                .short("c")
                                .long("config")
                                .value_name("FILE")
                                .help("Uses FILE as the config file of the server")
                                .takes_value(true)
                                .required(true)
                            ) 
                        
                        )
                        .get_matches();
    
    // see if we are generating a new config file
    if matches.is_present("new") {
        match matches.subcommand_matches("new") {
            Some(new_matches) => {

                // get the number of accesses we need
                let n_accesses = new_matches.value_of("number")
                    .unwrap()
                    .parse::<u64>()
                    .unwrap();
                // see if we need to do the file path too
                if new_matches.is_present("file") {
                    let fname = new_matches.value_of("file").unwrap();
                    config::generate_new(n_accesses, Some(fname.to_string()));
                } else {
                    config::generate_new(n_accesses, None);
                }

                // print that we succeeded
                log(LTYPE::Info, format!("Successfully generated new config with {} access UUIDs", n_accesses));
        
            },
            None => {
                log(LTYPE::Error, format!("Failed to parse new config arguments!"))
            }
        }
        std::process::exit(0);
        
    }

    // parse the config file
    if matches.is_present("run") {
        match matches.subcommand_matches("run") {
            Some(new_matches) => {
                let cfg = new_matches.value_of("config")
                    .unwrap();
                let cfg_file = match config::open_config(cfg.to_string()) {
                    Ok(a) => a,
                    Err(e) => {
                        log(LTYPE::Error, format!("Failed to open config file: {}", e));
                        std::process::exit(1);
                    }
                };

                // run the server
                run_server(cfg_file, None).await
            },
            None => panic!("Missing crit arg")
        }
    }
}