use warp::reject::Rejection;
use warp::reply::Reply;
use warp::http::Response;
type Result<T> = std::result::Result<T, Rejection>;

/// handle our data POST request
pub async fn post_handle(uuid: String, encdat: String) -> Result<impl Reply>{
    println!("[+] Triggered POST handler");
    let resp = Response::new(format!("Message: {}, {}", uuid, encdat));
    Ok(resp)
}

/// handle our data GET request
pub async fn get_handle(uuid: String, variable: String) -> Result<impl Reply>{
    println!("[+] Triggered GET handler");
    let resp = Response::new(format!("Message: {}, {}", uuid, variable));
    Ok(resp)
}