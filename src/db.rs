use axum::extract::FromRef;
use chrono::NaiveDate;
use shuttle_runtime::CustomError;
use sqlx::PgPool;

use crate::database::Itinerary;
use crate::AppState;

pub trait ItineraryRepository {
    fn get_itinerary(&self, id: i32) -> Result<Itinerary, CustomError>;
    fn create_itinerary(
        &self,
        name: &str,
        user_id: i32,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Itinerary, CustomError>;
    fn update_itinerary(&self, id: i32, itinerary: Itinerary) -> Result<Itinerary, CustomError>;
    fn delete_itinerary(&self, id: i32) -> Result<(), CustomError>;
    fn get_itineraries(&self) -> Result<Vec<Itinerary>, CustomError>;
    fn get_itineraries_by_user(&self, user_id: i32) -> Result<Vec<Itinerary>, CustomError>;
    fn add_stay_to_itinerary(&self, itinerary_id: i32) -> Result<(), CustomError>;
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}
