use axum::extract::{Path, State};

use anyhow::Result;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::auth::User;
use crate::error_handling::AppError;

#[tracing::instrument(name = "Get Itineraries", skip(db))]
pub async fn get_itineraries(user: User, State(db): State<PgPool>) -> impl IntoResponse {
    (StatusCode::NOT_IMPLEMENTED, "get_itineraries")
}

trait GetItineraryRespository {
    async fn get_itineraries(&self, user_id: usize) -> Result<Vec<Itinerary>>;
}

impl GetItineraryRespository for PgPool {
    async fn get_itineraries(&self, user_id: usize) -> Result<Vec<Itinerary>> {
        let inserted = sqlx::query_as!(
        Itinerary,
            r#"
                select 
                    itinerary_id as "id!",
                    name,
                    user_id
                from itineraries
                where user_id = $1
            "#,
            user_id as i32
        )
        .fetch_all(self)
        .await?;

        Ok(inserted)
    }
}


pub struct Itinerary {
    pub id: i32,
    pub name: String,
    pub user_id: i32
}
