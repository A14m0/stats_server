use warp::Filter;
use clap::{Arg, App, SubCommand};
mod handler;
mod config;
mod log;
use log::{
    log,
    LTYPE
};

async fn run_server() {
    log(LTYPE::Info, format!("Started web server"));

    // favicon path filter
    //let favico = warp::path("favicon.ico").and(warp::fs::file("favicon.ico"));

    // api filters
    let api_root = warp::path!("api"/..);
    let push = warp::path!("post" / String / String)
        .and(warp::post())
        .and_then(handler::post_handle);
    let get = warp::path!("get" / String / String)
        .and_then(handler::get_handle);

    let api = api_root
        .and(push.or(get));
    //let routes = favico
    //    .or(hi);
    // run the server, and point it to the keys
    warp::serve(api)
        .tls()
        .cert_path("./cert.pem")
        .key_path("./key.pem")
        .run(([127,0,0,1],8000))
        .await;
}

#[tokio::main]
async fn main() {
    // parse the CLI arguments
    let matches =  App::new("Stats Server")
                        .version("0.1")
                        .about("Serves up statistics")
                        .arg(Arg::with_name("config")
                            .short("c")
                            .long("config")
                            .value_name("FILE")
                            .help("Uses FILE as the config file of the server")
                            .takes_value(true)
                        ) 
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

    // run the server
    run_server().await
}