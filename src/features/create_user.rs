use axum::extract::State;

use anyhow::Result;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::auth::User;
use crate::error_handling::AppError;

#[tracing::instrument(name = "Create User", skip(db))]
pub async fn create_user(
    State(db): State<PgPool>,
    Json(create_user): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::NOT_IMPLEMENTED, "create_user"))
    // let user_id = db.create_user(create_user.into()).await?;
    // Ok((StatusCode::CREATED, format!("/itineraries/{}", user_id)))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    name: String,
    email: String,
    password: String,
}

impl<'a> From<&'a CreateUserRequest> for InsertUser<'a> {
    fn from(val: &'a CreateUserRequest) -> Self {
        InsertUser {
            name: &val.name,
            email: &val.email,
            password: &val.password,
        }
    }
}

struct InsertUser<'a> {
    name: &'a str,
    email: &'a str,
    password: &'a str,
}

trait CreateUserRespository {
    async fn create_user<'a>(&self, create_user: InsertUser<'a>) -> Result<i32>;
}

// impl CreateUserRespository for PgPool {
//     async fn create_user<'a>(&self, create_user: InsertUser<'a>) -> Result<i32> {
//         let inserted = sqlx::query!(
//             r#"
//             begin;
//                 WITH created_user as(
//                     insert into users ( name )
//                         values ( $1 )
//                         returning user_id
//                 )

//                 insert into user_email ( user_id, email )
//                 select
//                     user_id, $2
//                 from created_user;

//                 insert into user_password ( user_id, password )
//                 select
//                     user_id, $3
//                 from created_user;

//             commit;
//             "#,
//             create_user.name,
//             create_user.email,
//             create_user.password
//         )
//         .fetch_one(self)
//         .await?;

//         Ok(inserted.user_id as i32)
//     }
// }