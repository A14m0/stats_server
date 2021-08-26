use warp::reject::Rejection;
use warp::reply::Reply;
use warp::http::Response;
use futures::prelude::*;
use std::sync::MutexGuard;

use crate::log::{
    log,
    LTYPE
};
use crate::database;

type Result<T> = std::result::Result<T, Rejection>;



/// handle our data POST request
pub async fn post_handle(uuid: String, encdat: String, db: database::Db) -> Result<impl Reply>{
    
    log(LTYPE::Info, format!("Triggered POST handler"));
    let resp = Response::new(format!("Message: {}, {}", uuid, encdat));
    Ok(resp)
}

/// handle our data GET request
pub async fn get_handle(uuid: String, variable: String, db: database::Db) -> Result<impl Reply>{
    log(LTYPE::Info, format!("Triggered GET handler"));
    let resp = Response::new(format!("Message: {}, {}", uuid, variable));
    Ok(resp)
}

