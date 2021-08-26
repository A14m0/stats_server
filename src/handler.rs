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
pub async fn post_handle(uuid: String, encdat: String, db: database::Db) -> Result<impl Reply>{

    log(LTYPE::Info, format!("Triggered POST handler"));
    
    // add the new entry to the database and respnd with success
    let mut db_ref = db.lock().unwrap();
    (*db_ref).add_entry(
        database::DatabaseVar::new(
            uuid.clone(), 
            encdat.clone(), 
            gettime()
        )
    );

    let resp = Response::new(format!("Message: {}, {}", uuid, encdat));
    Ok(resp)
}

/// handle our data GET request
pub async fn get_handle(uuid: String, variable: String, db: database::Db) -> Result<impl Reply>{
    log(LTYPE::Info, format!("Triggered GET handler"));
    let resp = Response::new(format!("Message: {}, {}", uuid, variable));
    Ok(resp)
}

/// handle our data ADM request
pub async fn adm_handle(command: String, db: database::Db) -> Result<impl Reply>{
    log(LTYPE::Info, format!("Triggered ADM handler"));
    let resp = Response::new(format!(""));
    Ok(resp)
}

