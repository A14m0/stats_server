use warp::Filter;

#[tokio::main]
async fn main() {
    let hi = warp::path("hello")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| {
            format!("Hello {}, whose agent is {}", param, agent)
        });

    warp::serve(hi)
        .tls()
        .cert_path("./cert.pem")
        .key_path("./key.pem")
        .run(([127,0,0,1],8000))
        .await;
}
