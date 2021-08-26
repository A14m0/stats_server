use warp::reject::Rejection;
use warp::reply::Reply;
use warp::http::Response;

use crate::log::{
    log,
    LTYPE
};
use crate::database;
use crate::helpers::gettime;

type Result<T> = std::result::Result<T, Rejection>;



/// handle our data POST request
pub async fn post_handle(
    uuid: String, 
    data: database::DatabaseVar,
    db: database::Db) -> Result<impl Reply>{
    // remember, data is stored in the body of the message
    
    log(LTYPE::Info, format!("Triggered POST handler"));
    
    // add the new entry to the database and respnd with success
    let mut db_ref = db.lock().unwrap();
    (*db_ref).add_entry(
        data
    );

    let resp = warp::reply::with_status("OK", warp::http::StatusCode::default());//Response::new(format!("Message: {}, {}", uuid, encdat));
    Ok(resp)
}

/// handle our data GET request
pub async fn get_handle(uuid: String, 
    variable: String, 
    db: database::Db) -> Result<impl Reply>{
    
    log(LTYPE::Info, format!("Triggered GET handler"));
    
    // get a ref to the database
    let db_ref = db.lock().unwrap();
    for entry in (*db_ref).entries() {
        if entry.name() == variable {
            return Ok(
                warp::reply::json(
                    &entry
                )
            )
        }
    }


    let resp = warp::reply::json(&database::DatabaseVar::empty());
    Ok(resp)
}

/// handle our data ADM request
pub async fn adm_handle(
    command: String, 
    body: database::DatabaseVar, 
    db: database::Db) -> Result<impl Reply>{
    
        log(LTYPE::Info, format!("Triggered ADM handler"));
    let resp = Response::new(format!(""));
    Ok(resp)
}

