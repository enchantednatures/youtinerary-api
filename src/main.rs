mod auth;
mod db;

use axum::{routing::get, Router};
use serde::{Deserialize, Serialize};
use shuttle_runtime::CustomError;
use sqlx::{types::chrono::NaiveDate, PgPool};

pub use health_check::*;

mod health_check;

mod routes;
pub use routes::itineraries_router;

#[derive(Clone)]
pub struct AppState {
    pool: PgPool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateItineraryRequest {
    pub name: String,
    pub user_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateItineraryRequest {
    pub name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteItineraryRequest {
    pub id: i32,
}
mod database {
    use shuttle_runtime::CustomError;
    use sqlx::types::chrono::NaiveDate;

    pub use super::models::*;

    #[async_trait::async_trait]
    pub trait ItineraryRepository {
        async fn get_itinerary(&self, id: i32) -> Result<Itinerary, CustomError>;
        async fn create_itinerary(
            &self,
            name: &str,
            user_id: i32,
            start_date: NaiveDate,
            end_date: NaiveDate,
        ) -> Result<Itinerary, CustomError>;
        async fn update_itinerary(
            &self,
            id: i32,
            itinerary: Itinerary,
        ) -> Result<Itinerary, CustomError>;
        async fn delete_itinerary(&self, id: i32) -> Result<(), CustomError>;
        async fn get_itineraries(&self) -> Result<Vec<Itinerary>, CustomError>;
    }
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

mod models {
    use serde::{Deserialize, Serialize};
    use sqlx::{
        types::chrono::{DateTime, NaiveDate, Utc},
        FromRow,
    };

    #[derive(FromRow, Serialize, Deserialize)]
    pub struct User {
        pub id: i32,
        pub name: String,
        pub email: String,
    }

    #[derive(FromRow, Serialize, Deserialize)]
    pub struct Itinerary {
        pub id: i32,
        pub name: String,
        pub user_id: i32,
        pub created_at: DateTime<Utc>,
        pub start_date: NaiveDate,
        pub end_date: NaiveDate,
    }

    #[derive(sqlx::Type, Serialize, Deserialize)]
    #[sqlx(type_name = "itinerary_status", rename_all = "lowercase")]
    pub enum ItineraryStatus {
        Draft,
        Published,
        Archived,
    }

    #[derive(sqlx::Type, Serialize, Deserialize)]
    #[sqlx(type_name = "itinerary_share_type", rename_all = "lowercase")]
    pub enum ItineraryShareType {
        Editor,
        Viewer,
    }

    #[derive(FromRow, Serialize, Deserialize)]
    pub struct ItineraryShare {
        pub id: i32,
        pub itinerary_id: i32,
        pub user_id: i32,
        pub share_type: ItineraryShareType,
        pub share_message: String,
    }

    #[derive(FromRow, Serialize, Deserialize)]
    pub struct ItineraryItem {
        pub id: i32,
        pub name: String,
        pub itinerary_id: i32,
    }

    #[derive(sqlx::Type, Serialize, Deserialize)]
    pub struct ItineraryStay {
        pub itinerary_id: i32,
        pub stay_id: i32,
    }

    #[derive(FromRow, Serialize, Deserialize)]
    pub struct Stay {
        pub id: i32,
        pub summary: i32,
        pub start_date: NaiveDate,
        pub end_date: NaiveDate,
        pub location: String,
        pub notes: String,
    }

    #[derive(FromRow, Serialize, Deserialize)]
    pub struct Activity {
        pub id: i32,
        pub summary: i32,
        pub start_date: NaiveDate,
        pub end_date: NaiveDate,
        pub location: String,
        pub notes: String,
    }

    #[derive(sqlx::Type, Serialize, Deserialize)]
    #[sqlx(type_name = "travel_leg_type", rename_all = "lowercase")]
    pub enum TravelLegType {
        Flight,
        Train,
        Bus,
        Car,
        Ferry,
        Other,
    }

    #[derive(FromRow, Serialize, Deserialize)]
    pub struct TravelLeg {
        pub id: i32,
        pub from: i32,
        pub to: i32,
        pub start: DateTime<Utc>,
        pub end: DateTime<Utc>,
        pub tz_start: String,
        pub tz_end: String,
    }
}
