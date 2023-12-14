use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use sqlx::PgPool;

use crate::{AppState, CreateItineraryRequest};

#[tracing::instrument(name = "Get Itinerary", skip(db))]
pub async fn get_itinerary(State(db): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::OK, "get_itinerary")
}

#[tracing::instrument(name = "Create Itinerary", skip(db, create_itinerary))]
pub async fn create_itinerary(
    State(db): State<PgPool>,
    Json(create_itinerary): Json<CreateItineraryRequest>,
) -> impl IntoResponse {
    (StatusCode::CREATED, Json(create_itinerary))
}

#[tracing::instrument(name = "Put Itinerary", skip(db))]
pub async fn put_itinerary(State(db): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "put_itinerary")
}

#[tracing::instrument(name = "Get Itineraries", skip(db))]
pub async fn get_itineraries(State(db): State<PgPool>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "get_itineraries")
}

#[tracing::instrument(name = "Get Itineraries", skip(db))]
pub fn delete_itinerary(State(db): State<PgPool>, Path(id): Path<i32>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "delete_itinerary")
}

pub fn itineraries_router() -> Router<AppState> {
    Router::new()
        .route("/itineraries", get(get_itineraries).post(create_itinerary))
        .route(
            "/itineraries/:id",
            get(get_itinerary).put(put_itinerary), // .delete(delete_itinerary),
        )
}
