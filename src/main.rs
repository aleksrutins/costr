use std::error::Error;

use tracing_subscriber::fmt::format::FmtSpan;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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

    let port = std::env::var("PORT")
        .unwrap_or("8080".to_owned())
        .parse()?;

    warp::serve(routes)
        .bind(([0, 0, 0, 0], port))
        .await;
    Ok(())
}
