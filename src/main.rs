use axum::{routing::get, Router, ServiceExt, extract::Query};
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[derive(Deserialize)]
struct N{st: String}

#[tokio::main]
async fn main(){
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "example_tracing_aka_logging=debug,tower_http=debug",
        )
    }
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", get(|Query(_): Query<N>| async {"hello wordl!"}));
    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(ServiceBuilder::new().layer(TraceLayer::new_for_http()).service(app).into_make_service())
        .await
        .unwrap();
}
