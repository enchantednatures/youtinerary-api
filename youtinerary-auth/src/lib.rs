mod error;

use anyhow::{Context, Result};
use error::AuthError;

use async_session::Session;
use axum::extract::{Query, State};
use axum_extra::typed_header::TypedHeaderRejectionReason;
use axum_extra::TypedHeader;
use hyper::HeaderMap;
use oauth2::IntrospectionUrl;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, CsrfToken, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, RevocationUrl,
    Scope, TokenResponse, TokenUrl,
};

use axum::http::header::SET_COOKIE;
use axum::response::{IntoResponse, Redirect, Response};

use axum::http::header;
use axum::http::request::Parts;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    RequestPartsExt,
};

use redis::AsyncCommands;

use serde::{Deserialize, Serialize};
use tracing::field::display;
use tracing::Span;
static COOKIE_NAME: &str = "SESSION";

// #[derive(Clone)]
// pub struct AuthState {
//     redis: redis::Client,
//     oauth_client: BasicClient,
//     reqwest_client: reqwest::Client,
// }

#[derive(Debug, Deserialize)]
pub struct AuthSettings {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_url: String,
    pub token_url: String,
    pub auth_url: String,
    pub introspection_url: String,
    pub revocation_url: String,
}

#[derive(Serialize, Deserialize)]
struct Oath2State {
    pkce_code_verifier_secret: String,
    return_url: String,
}

trait SessionManager {
    async fn get_session<'a>(&self, session_id: &'a str) -> Result<Option<Session>>;
    async fn set_session(&self, session: &Session) -> Result<String>;
    async fn set_verifier(&self, csrf: &CsrfToken, state: &Oath2State) -> Result<()>;
    async fn get_verifier(&self, csrf: &CsrfToken) -> Result<Oath2State>;
}

impl SessionManager for redis::Client {
    async fn get_session<'a>(&self, session_id: &'a str) -> Result<Option<Session>> {
        let mut con = self.get_async_connection().await?;
        let session: String = con.get(session_id).await?;
        let session: Session = serde_json::from_str(&session)?;
        con.expire(session.id(), 300).await?;
        Ok(Some(session))
    }

    async fn set_session(&self, session: &Session) -> Result<String> {
        let mut con = self.get_async_connection().await?;
        con.set(session.id(), serde_json::to_string(session)?)
            .await?;
        con.expire(session.id(), 300).await?;
        Ok(session.id().to_string())
    }

    async fn set_verifier(&self, csrf: &CsrfToken, state: &Oath2State) -> Result<()> {
        let mut con = self.get_async_connection().await?;
        con.set(csrf.secret(), serde_json::to_string(state)?)
            .await?;
        con.expire(csrf.secret(), 300).await?;
        Ok(())
    }

    async fn get_verifier(&self, csrf: &CsrfToken) -> Result<Oath2State> {
        let mut con = self.get_async_connection().await?;
        let state: String = con.get_del(csrf.secret()).await?;
        let state: Oath2State = serde_json::from_str(&state)?;
        Ok(state)
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthentikUser {
    pub email: String,
    pub sub: String,
}

pub struct AuthRedirect;

impl From<anyhow::Error> for AuthRedirect {
    fn from(_value: anyhow::Error) -> Self {
        Self {}
    }
}

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/authorize").into_response()
    }
}
#[async_trait]
impl<S> FromRequestParts<S> for AuthentikUser
where
    redis::Client: FromRef<S>,
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = AuthRedirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = redis::Client::from_ref(state);

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

        let session = store
            .get_session(session_cookie)
            .await?
            .ok_or(AuthRedirect)?;

        let user = session.get::<AuthentikUser>("user").ok_or(AuthRedirect)?;

        Ok(user)
    }
}

#[tracing::instrument(name = "Protected area")]
pub async fn protected(user: AuthentikUser) -> impl IntoResponse {
    format!(
        "Welcome to the protected area :)\nHere's your info:\n{:?}",
        user
    )
}

#[tracing::instrument(name = "Login authorized", skip(store, oauth_client))]
pub async fn login_authorized(
    Query(query): Query<AuthRequest>,
    State(store): State<redis::Client>,
    State(client): State<reqwest::Client>,
    State(oauth_client): State<BasicClient>,
) -> Result<impl IntoResponse, AuthError> {
    let AuthRequest { code, state } = query;

    Span::current()
        .record("code", &display(&code))
        .record("state", &display(&state));

    let state = CsrfToken::new(state);
    let code = AuthorizationCode::new(code);
    let Oath2State {
        pkce_code_verifier_secret,
        return_url,
    } = store.get_verifier(&state).await.unwrap();

    let pkce_code_verifier = PkceCodeVerifier::new(pkce_code_verifier_secret);

    let token_response = oauth_client
        .exchange_code(code)
        .set_pkce_verifier(pkce_code_verifier)
        .request_async(async_http_client)
        .await
        .map_err(|err| match err {
            oauth2::RequestTokenError::ServerResponse(server_response) => {
                format!("Server Error: {}", server_response)
            }
            oauth2::RequestTokenError::Request(request_error) => {
                format!("Request Error: {}", request_error)
            }
            oauth2::RequestTokenError::Parse(s, v) => format!("Parse Error: {} {:?}", s, v),
            oauth2::RequestTokenError::Other(o) => format!("OAuth: exchange_code failure: {}", o),
        })?;

    let access_token_secret = token_response.access_token().secret();
    let url = oauth_client.introspection_url().unwrap().url().as_str();

    let user_data_response = client
        .get(url)
        .bearer_auth(access_token_secret)
        .send()
        .await?;

    dbg!(&user_data_response);

    let user_data = user_data_response.text().await?;

    dbg!(&user_data);

    let user_data: AuthentikUser = serde_json::from_str(&user_data)?;

    // Create a new session filled with user data
    let mut session = Session::new();
    session.insert("user", &user_data).unwrap();

    // Store session and get corresponding cookie
    let cookie = store.set_session(&session).await.unwrap();

    // Build the cookie
    let cookie = format!("{}={}; SameSite=Lax; Path=/", COOKIE_NAME, cookie);

    // Set cookie
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().unwrap());

    Ok((headers, Redirect::to(&return_url)))
}

#[tracing::instrument(name = "Authorize", skip(store, oauth_client))]
pub async fn authorize(
    State(store): State<redis::Client>,
    State(oauth_client): State<BasicClient>,
) -> impl IntoResponse {
    let (pkce_code_challenge, pkce_code_verifier) = PkceCodeChallenge::new_random_sha256();
    let (auth_url, csrf_token) = oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("identify".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("openid".to_string()))
        .set_pkce_challenge(pkce_code_challenge)
        .url();

    let state = Oath2State {
        pkce_code_verifier_secret: pkce_code_verifier.secret().to_string(),
        return_url: "/".to_string(),
    };

    store.set_verifier(&csrf_token, &state).await.unwrap();

    Redirect::to(auth_url.as_str())
}

impl TryFrom<AuthSettings> for BasicClient {
    type Error = anyhow::Error;

    fn try_from(auth_settings: AuthSettings) -> Result<Self> {
        Ok(BasicClient::new(
            ClientId::new(auth_settings.client_id),
            Some(ClientSecret::new(auth_settings.client_secret)),
            AuthUrl::new(auth_settings.auth_url)
                .context("failed to create new authorization server URL")?,
            Some(
                TokenUrl::new(auth_settings.token_url)
                    .context("failed to create new token endpoint URL")?,
            ),
        )
        .set_revocation_uri(RevocationUrl::new(auth_settings.revocation_url)?)
        .set_introspection_uri(IntrospectionUrl::new(auth_settings.introspection_url)?)
        .set_redirect_uri(
            RedirectUrl::new(auth_settings.redirect_url)
                .context("failed to create new redirection URL")?,
        ))
    }
}
