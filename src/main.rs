mod auth;
mod db;
mod models;
use axum::{routing::get, Router};
pub use models::*;
use serde::{Deserialize, Serialize};
use shuttle_runtime::CustomError;
use sqlx::{types::chrono::NaiveDate, PgPool};

mod database;
pub use database::*;
pub use health_check::*;

mod health_check;

mod routes;
pub use routes::itineraries_router;

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
}



// #[async_trait::async_trait]
// impl ItineraryRepository for PgPool {}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let state = AppState { pool };
    let router = Router::new()
        .route("/", get(health_check))
        .route("/health_check", get(health_check))
        .nest("/api/v0", itineraries_router())
        // .route("", get(retrieve))
        .with_state(state);

    Ok(router.into())
}
