mod deploy;
use std::{
    path::PathBuf,
};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, TypedHeader,
    },
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router, ServiceExt,
};
use once_cell::sync::OnceCell;
use serde::Deserialize;
use tokio::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};

#[derive(Deserialize)]
struct N {
    st: String,
}

static paths: OnceCell<Vec<String>> = OnceCell::new();
static used_ports: OnceCell<Mutex<Vec<i32>>> = OnceCell::new();

#[tokio::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var(
            "RUST_LOG",
            "tracing_aka_logging=debug,websockets=debug,tower_http=debug",
        )
    }
    let (res, ports) = deploy::getpaths(PathBuf::from("."))?;
    used_ports.set(Mutex::new(ports)).unwrap();
    paths.set(res).map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "unable to set project paths: 0x1"))?;
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(|Query(_): Query<N>| async { "hello wordl!" }))
        .route("/ws", get(ws_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );
    axum::Server::bind(&"127.0.0.1:8080".parse().unwrap())
        .serve(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .service(app)
                .into_make_service(),
        )
        .await
        .unwrap();
    Ok(())
}

async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
) -> impl IntoResponse {
    if let Some(TypedHeader(user_agent)) = user_agent {
        tracing::debug!("`{}` connected", user_agent.as_str());
    }

    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    loop {
        if let Some(msg) = socket.recv().await {
            if let Ok(msg) = msg {
                match msg {
                    Message::Text(t) => {
                        tracing::info!("client sent str: {:?}", t);
                    }
                    Message::Binary(_) => {
                        tracing::info!("client sent binary data");
                    }
                    Message::Ping(_) => {
                        tracing::info!("socket ping");
                    }
                    Message::Pong(_) => {
                        tracing::info!("socket pong");
                    }
                    Message::Close(_) => {
                        tracing::info!("client disconnected");
                        return;
                    }
                }
            } else {
                println!("client disconnected");
                return;
            }
        }
    }
}
