use anyhow::Result;

use axum::response::{IntoResponse, Redirect, Response};

use axum::http::header;
use axum::http::request::Parts;
use axum::{
    async_trait,
    extract::{rejection::TypedHeaderRejectionReason, FromRequestParts, TypedHeader},
    headers, RequestPartsExt,
};
use serde::Deserialize;

use serde::Serialize;

static COOKIE_NAME: &str = "SESSION";

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: usize,
}

pub struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/authorize").into_response()
    }
}
#[async_trait]
impl<S> FromRequestParts<S> for User
where
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = AuthRedirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let cookies = parts
            .extract::<TypedHeader<headers::Cookie>>()
            .await
            .map_err(|e| match *e.name() {
                header::COOKIE => match e.reason() {
                    TypedHeaderRejectionReason::Missing => AuthRedirect,
                    _ => panic!("unexpected error getting Cookie header(s): {}", e),
                },
                _ => panic!("unexpected error getting cookies: {}", e),
            })?;
        let session_cookie = cookies.get(COOKIE_NAME).ok_or(AuthRedirect)?;

        Ok(User { id: 0 })
    }
}
