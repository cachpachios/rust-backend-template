use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ErrorResponse {
    pub reason: Cow<'static, str>,
    #[serde(skip)]
    pub status_code: StatusCode,
}

impl ErrorResponse {
    pub fn new<T>(status_code: StatusCode, reason: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self {
            reason: reason.into(),
            status_code,
        }
    }

    /// Constructs a BadRequest (400) error response.
    pub fn bad_request<T>(reason: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self::new(StatusCode::BAD_REQUEST, reason)
    }

    /// Constructs a Unauthorized (401) error response.
    pub fn unauthorized<T>(reason: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self::new(StatusCode::UNAUTHORIZED, reason)
    }

    /// Constructs a Forbidden (403) error response.
    pub fn forbidden<T>(reason: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self::new(StatusCode::FORBIDDEN, reason)
    }

    /// Constructs a NotFound (404) error response.
    pub fn not_found<T>(reason: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self::new(StatusCode::NOT_FOUND, reason)
    }

    /// Constructs a Conflict (409) error response.
    pub fn conflict<T>(reason: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self::new(StatusCode::CONFLICT, reason)
    }

    /// Constructs a ServiceUnavailable (503) error response.
    pub fn service_unavailable<T>(reason: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        Self::new(StatusCode::SERVICE_UNAVAILABLE, reason)
    }

    /// Constructs a InternalServerError (500) error response.
    /// IMPORTANT! Reason is only shown in debug builds.
    pub fn internal_server_error<T>(reason: T) -> Self
    where
        T: Into<Cow<'static, str>>,
    {
        #[cfg(debug_assertions)]
        {
            Self::new(StatusCode::INTERNAL_SERVER_ERROR, reason)
        }
        #[cfg(not(debug_assertions))]
        {
            let _ = reason; // Consume the parameter to avoid unused warnings
            Self::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        // For 500 errors in release mode, don't include any reason
        if self.status_code == StatusCode::INTERNAL_SERVER_ERROR {
            #[cfg(debug_assertions)]
            {
                let body = Json(&self);
                (self.status_code, body).into_response()
            }
            #[cfg(not(debug_assertions))]
            {
                // Return just the status code without any body for 500 errors in release
                self.status_code.into_response()
            }
        } else {
            let body = Json(&self);
            (self.status_code, body).into_response()
        }
    }
}
