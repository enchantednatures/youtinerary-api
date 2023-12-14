use axum::extract::FromRef;
use chrono::NaiveDate;
use shuttle_runtime::CustomError;
use sqlx::PgPool;

use crate::database::Itinerary;
use crate::AppState;

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

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}
