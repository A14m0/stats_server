use warp::Filter;
mod handler;

#[tokio::main]
async fn main() {

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
