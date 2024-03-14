use plexo_sdk::errors::sdk::SDKError;
use poem::{error::ResponseError, http::StatusCode};
use thiserror::Error;

// use poem::http::{HeaderMap, StatusCode};
#[derive(Error, Debug)]
pub enum PlexoAppError {
    #[error("Authorization token not provided")]
    MissingAuthorizationToken,
    #[error("Invalid authorization token")]
    InvalidAuthorizationToken,
    #[error("Email already in use")]
    EmailAlreadyInUse,
    #[error("Password isn't valid")]
    InvalidPassword,
    #[error("Email not found")]
    EmailNotFound,
    #[error("Email already exists")]
    EmailAlreadyExists,

    #[error("SDKError error")]
    SDKError(#[from] SDKError),

    #[error("Poem error")]
    NotFoundPoemError(#[from] poem::error::NotFoundError),

    #[error("JSONWebToken error")]
    JSONWebTokenError(#[from] jsonwebtoken::errors::Error),
    // #[error("Resend error")]
    // ResendError(#[from] resend_rs::error::Error),
}

impl ResponseError for PlexoAppError {
    fn status(&self) -> StatusCode {
        match self {
            PlexoAppError::MissingAuthorizationToken => StatusCode::UNAUTHORIZED,
            PlexoAppError::InvalidAuthorizationToken => StatusCode::UNAUTHORIZED,
            PlexoAppError::EmailAlreadyInUse => StatusCode::BAD_REQUEST,
            PlexoAppError::InvalidPassword => StatusCode::BAD_REQUEST,
            PlexoAppError::EmailNotFound => StatusCode::BAD_REQUEST,
            PlexoAppError::EmailAlreadyExists => StatusCode::BAD_REQUEST,
            PlexoAppError::SDKError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PlexoAppError::NotFoundPoemError(_) => StatusCode::NOT_FOUND,
            PlexoAppError::JSONWebTokenError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
