use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;

#[tokio::main]
async fn main() {
    let filter = std::env::var("RUST_LOG").unwrap_or_else(|_| "tracing=info,warp=debug".to_owned());

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {name}!"))
        .with(warp::trace::named("hello"));

    let routes = hello
        .with(warp::trace::request());
    
    warp::serve(routes)
        .bind(([127, 0, 0, 1], 8080))
        .await;
}
