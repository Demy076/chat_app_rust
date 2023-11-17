use axum::{
    extract::State,
    http::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use axum_extra::extract::CookieJar;
use once_cell::sync::Lazy;
use serde::Serialize;

use crate::{prisma_client::client::user, shared::arc_clients::State as app_state};
static ALLOWED_ROUTES: Lazy<Vec<&str>> = Lazy::new(|| vec!["/create"]);
static CSRF_METHODS: Lazy<Vec<axum::http::Method>> = Lazy::new(|| {
    vec![
        axum::http::Method::POST,
        axum::http::Method::PUT,
        axum::http::Method::DELETE,
    ]
});

enum AuthError {
    AlreadyAuthenticated,
    NotAuthenticated,
    InternalError,
}

#[derive(Serialize)]
pub struct AuthenticationErrorResponse {
    pub success: bool,
    pub http_code: u16,
    pub error: String,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let error_message: String = match self {
            AuthError::InternalError => "Internal Server Error".to_string(),
            AuthError::AlreadyAuthenticated => "Already Authenticated".to_string(),
            AuthError::NotAuthenticated => "Not Authenticated".to_string(),
        };

        let (status, error_response) = match self {
            AuthError::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(AuthenticationErrorResponse {
                    success: false,
                    http_code: 500,
                    error: error_message,
                }),
            ),
            AuthError::AlreadyAuthenticated => (
                StatusCode::BAD_REQUEST,
                Json(AuthenticationErrorResponse {
                    success: false,
                    http_code: 400,
                    error: error_message,
                }),
            ),
            AuthError::NotAuthenticated => (
                StatusCode::UNAUTHORIZED,
                Json(AuthenticationErrorResponse {
                    success: false,
                    http_code: 401,
                    error: error_message,
                }),
            ),
        };

        let body = error_response.into_response().into_body();

        let mut response = Response::new(body);

        *response.status_mut() = status;

        response.headers_mut().insert(
            "Content-Type",
            "application/json; charset=utf-8".parse().unwrap(),
        );

        if status == StatusCode::UNAUTHORIZED && response.headers().contains_key("Cookie") {
            response.headers_mut().insert(
                "Set-Cookie",
                "session=; HttpOnly; Secure; SameSite=Lax".parse().unwrap(),
            );
        }

        response
    }
}
pub async fn is_authed<B>(
    State(state): State<app_state>,
    jar: CookieJar,
    mut request: Request<B>,
    next: Next<B>,
) -> Response {
    let cookie = jar.get("session");
    if cookie.is_none() {
        if ALLOWED_ROUTES.contains(&request.uri().path()) {
            return next.run(request).await;
        }
        return AuthError::NotAuthenticated.into_response();
    }
    let cookie = cookie.unwrap();
    let user = state
        .prisma_client
        .user()
        .find_first(vec![user::token::equals(cookie.value().to_string())])
        .exec()
        .await;
    let user = match user {
        Ok(user) => user,
        Err(_) => {
            if ALLOWED_ROUTES.contains(&request.uri().path()) {
                return next.run(request).await;
            }
            return AuthError::InternalError.into_response();
        }
    };

    if user.is_none() {
        if ALLOWED_ROUTES.contains(&request.uri().path()) {
            return next.run(request).await;
        }
        return AuthError::NotAuthenticated.into_response();
    }
    if ALLOWED_ROUTES.contains(&request.uri().path()) {
        return AuthError::AlreadyAuthenticated.into_response();
    }

    let user = user.unwrap();
    if CSRF_METHODS.contains(request.method()) && !ALLOWED_ROUTES.contains(&request.uri().path()) {
        let csrf_header = match request.headers().get("X-CSRF-TOKEN") {
            Some(csrf_header) => match csrf_header.to_str() {
                Ok(csrf_header) => csrf_header,
                Err(_) => return AuthError::NotAuthenticated.into_response(),
            },
            None => return AuthError::NotAuthenticated.into_response(),
        };
        if csrf_header != user.csrf_token {
            return AuthError::NotAuthenticated.into_response();
        }
    }
    request.extensions_mut().insert(user);
    next.run(request).await
}
