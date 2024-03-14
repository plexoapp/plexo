use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct GithubCallbackParams {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthenticationResponse {
    pub access_token: String,
    pub token_type: Option<String>,
    pub scope: Option<String>,
}

pub struct PlexoAuthToken(pub String);

#[derive(Debug, Deserialize)]
pub struct EmailLoginParams {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct EmailRegisterParams {
    pub email: String,
    pub name: String,
    pub password: String,
}
