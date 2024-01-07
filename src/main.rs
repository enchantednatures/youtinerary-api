mod auth;
pub mod error_handling;
mod features;
mod models;
use axum::extract::FromRef;
use axum::{routing::get, Router};
pub use models::*;
use shuttle_runtime::CustomError;
use sqlx::PgPool;

pub use health_check::*;

mod health_check;

mod routes;
pub use routes::itineraries_router;

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

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
