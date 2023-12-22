use shuttle_runtime::CustomError;
use sqlx::types::chrono::NaiveDate;
use sqlx::{query_as, query_file, PgPool};

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

#[async_trait::async_trait]
pub trait CreateItineraryStayRepsitory {
    async fn create_itinerary_stay(
        &self,
        itinerary_id: i32,
        stay_id: i32,
    ) -> Result<ItineraryStay, CustomError>;
}

#[async_trait::async_trait]
pub trait CreateStayRepository {
    async fn create_stay(
        &self,
        name: &str,
        description: &str,
        user_id: i32,
        location: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Stay, CustomError>;
}

#[async_trait::async_trait]
impl CreateStayRepository for PgPool {
    async fn create_stay(
        &self,
        name: &str,
        description: &str,
        user_id: i32,
        location: &str,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Stay, CustomError> {
        // todo!()
        let created_stay = query_as!(
            Stay,
            r#"INSERT INTO stays(summary , start_date , end_date , location , notes )
            VALUES ( $1 , $2 , $3 , $4 , $5 )
            "#,
            description, start_date, end_date, location, name
        );
    }
}
