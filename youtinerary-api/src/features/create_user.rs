use axum::extract::State;

use anyhow::Result;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::error_handling::AppError;

#[tracing::instrument(name = "Create User", skip(db))]
pub async fn create_user(
    State(db): State<PgPool>,
    Json(create_user): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user_id = db.create_user(&InsertUser::from(&create_user)).await?;
    Ok((StatusCode::CREATED, format!("/itineraries/{}", user_id)))
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

enum CreateUserError{
    DuplicateUser,
}

struct InsertUser<'a> {
    name: &'a str,
    email: &'a str,
    password: &'a str,
}

trait CreateUserRespository {
    async fn create_user<'a>(&self, create_user: &'a InsertUser<'a>) -> Result<i32>;
}

impl CreateUserRespository for PgPool {
    async fn create_user<'a>(&self, create_user: &'a InsertUser<'a>) -> Result<i32> {
        let mut transaction = self.begin().await?;
        let inserted = sqlx::query!(
            r#"
                insert into users ( name )
                values ( $1 )
                returning user_id;
            "#,
            create_user.name
        )
        .fetch_one(&mut *transaction)
        .await?;

        let _ = sqlx::query!(
            r#"
                insert into user_email ( user_id, email )
                values( $1, $2 );
            "#,
            inserted.user_id,
            create_user.email
        )
        .execute(&mut *transaction)
        .await?;

        let _ = sqlx::query!(
            r#"
                insert into user_password ( user_id, password )
                values( $1, $2 );
            "#,
            inserted.user_id,
            create_user.password
        )
        .execute(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(inserted.user_id)
    }
}
