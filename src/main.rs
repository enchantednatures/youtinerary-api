use axum::{routing::get, Router};
use shuttle_runtime::CustomError;
use sqlx::PgPool;

async fn hello_world() -> &'static str {
    "Hello, world!"
}

#[derive(Clone)]
struct MyState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!()
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let state = MyState { pool };
    let router = Router::new()
        .route("/", get(hello_world))
        // .route("", get(retrieve))
        .with_state(state);

    Ok(router.into())
}

mod models {
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;

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
    }

    #[derive(sqlx::Type, Serialize, Deserialize)]
    pub enum ItineraryStatus {
        Draft,
        Published,
    }

    #[derive(sqlx::Type, Serialize, Deserialize)]
    pub enum ItineraryType {
        Public,
        Private,
    }

    #[derive(sqlx::Type, Serialize, Deserialize)]
    pub enum ItineraryShareType {
        Owner,
        Editor,
        Viewer,
    }

    #[derive(FromRow, Serialize, Deserialize)]
    pub struct ItineraryShare {
        pub id: i32,
        pub itinerary_id: i32,
        pub user_id: i32,
    }

    #[derive(FromRow, Serialize, Deserialize)]
    pub struct Item {
        pub id: i32,
        pub name: String,
        pub itinerary_id: i32,
    }
}
