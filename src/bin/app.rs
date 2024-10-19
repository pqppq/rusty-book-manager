use adapter::database::connect_database_with;

use anyhow::Context;
use anyhow::Result;
use api::route::{book::build_book_routes, health::build_health_check_routes};
use axum::http::Method;
use axum::Router;
use registry::AppRegistry;
use shared::{
    config::AppConfig,
    env::{which, Environment},
};
use std::net::{Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    init_logger()?;
    bootstrap().await
}

fn init_logger() -> Result<()> {
    let log_level = match which() {
        Environment::Development => "debug",
        Environment::Production => "info",
    };

    // set log level
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| log_level.into());

    // set log format
    let subscriber = tracing_subscriber::fmt::layer()
        .with_file(true)
        .with_line_number(true)
        .with_target(false);

    tracing_subscriber::registry()
        .with(subscriber)
        .with(env_filter)
        .try_init()?;

    Ok(())
}

async fn bootstrap() -> Result<()> {
    let app_config = AppConfig::new()?;
    let pool = connect_database_with(&app_config.database);

    let registry = AppRegistry::new(pool);
    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);
    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(
            DefaultOnResponse::new()
                .level(Level::INFO)
                .latency_unit(LatencyUnit::Millis),
        );

    let app = Router::new()
        .merge(build_health_check_routes())
        .merge(build_book_routes())
        .layer(cors)
        .layer(tracing_layer)
        .with_state(registry);

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8080);
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app)
        .await
        // wrap the error value with additional context.
        .context("Unexpected error happened in server")
        // calls a function with a reference to the contained value if Err
        .inspect_err(|e| {
            tracing::error!(
                error.cause_chain = ?e,
                error.message = %e,
                "Unexpected error"
            )
        })
}
