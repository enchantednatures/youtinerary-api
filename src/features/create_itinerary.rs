use axum::extract::{Path, State};

use anyhow::Result;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error_handling::AppError;

#[tracing::instrument(name = "Create Itinerary", skip(db))]
pub async fn create_itinerary(
    State(db): State<PgPool>,
    Path(user_id): Path<usize>,
    Json(create_itinerary): Json<CreateItineraryRequest>,
) -> Result<impl IntoResponse, AppError> {
    let itinerary_id = db
        .create_itinerary((user_id, create_itinerary).into())
        .await?;
    Ok((
        StatusCode::CREATED,
        format!("/itineraries/{}", itinerary_id),
    ))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateItineraryRequest {
    name: String,
}

impl Into<InsertItinerary> for (usize, CreateItineraryRequest) {
    fn into(self) -> InsertItinerary {
        InsertItinerary {
            user_id: self.0,
            name: self.1.name,
        }
    }
}

struct InsertItinerary {
    user_id: usize,
    name: String,
}

trait CreateItineraryRespository {
    async fn create_itinerary(&self, create_itinerary: InsertItinerary) -> Result<usize>;
}

impl CreateItineraryRespository for PgPool {
    async fn create_itinerary(&self, create_itinerary: InsertItinerary) -> Result<usize> {
        let inserted = sqlx::query!(
            r#"
            INSERT INTO itineraries (user_id, name)
            VALUES ($1, $2)
            RETURNING itinerary_id
            "#,
            create_itinerary.user_id as i32,
            create_itinerary.name,
        )
        .fetch_one(self)
        .await?;

        Ok(inserted.itinerary_id as usize)
    }
}
