use std::net::{Ipv4Addr, SocketAddr};

use anyhow::{Ok, Result};
use axum::{http::StatusCode, routing::get, Router};
use tokio::net::TcpListener;

pub async fn health() -> StatusCode {
    StatusCode::OK
}

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new()
        .route("/health", get(health))
        .route("/hello", get(hello_world));

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    Ok(axum::serve(listener, app).await?)
}
