mod auth;
pub mod error_handling;
mod features;
mod health_check;
mod models;
mod routes;
use anyhow::Context;
use anyhow::Result;
use axum::extract::FromRef;
use axum::{routing::get, Router};
pub use health_check::*;
pub use models::*;
pub use routes::itineraries_router;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{EnvFilter, Registry};

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
    redis: redis::Client,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

async fn connect_database(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("can't connect to database")
}

#[tokio::main]
async fn main() -> Result<()> {
    let formatting_layer = BunyanFormattingLayer::new("youtinerary".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(JsonStorageLayer)
        .with(EnvFilter::new("info"))
        .with(formatting_layer);

    tracing::subscriber::set_global_default(subscriber).unwrap();
    let pool: PgPool = connect_database(&std::env::var("DATABASE_URL")?).await;
    let redis = redis::Client::open(std::env::var("REDIS_URL")?).unwrap();
    sqlx::migrate!().run(&pool).await?;

    let state = AppState { pool, redis };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .context("failed to bind TcpListener")
        .unwrap();

    let router = Router::new()
        .route("/", get(health_check))
        .route("/health_check", get(health_check))
        .nest("/api/v0", itineraries_router())
        // .route("", get(retrieve))
        .with_state(state);

    axum::serve(listener, router).await.unwrap();
    Ok(())
}
