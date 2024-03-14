use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use plexo_sdk::resources::members::member::Member;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::errors::app::PlexoAppError;

// use crate::sdk::member::Member;

#[derive(Default, Clone)]
pub struct JWTEngine {
    access_token_secret: String,
    // refresh_token_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlexoAuthTokenClaims {
    iss: String,
    aud: String,
    sub: String,
    exp: usize,
}

impl PlexoAuthTokenClaims {
    pub fn member_id(&self) -> Uuid {
        Uuid::parse_str(&self.sub).unwrap()
    }
}

impl JWTEngine {
    pub fn new(access_token_secret: String, _refresh_token_secret: String) -> Self {
        Self {
            access_token_secret,
            // refresh_token_secret,
        }
    }

    pub fn create_session_token(&self, member: &Member) -> Result<String, PlexoAppError> {
        let claims = PlexoAuthTokenClaims {
            iss: "Plexo".to_string(),
            aud: "session.plexo.app".to_string(),
            sub: member.id.to_string(),
            exp: (Utc::now() + chrono::Duration::try_days(7).unwrap()).timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.access_token_secret.as_ref()),
        )?;

        Ok(token)
    }

    pub fn decode_session_token(&self, token: &str) -> Result<PlexoAuthTokenClaims, PlexoAppError> {
        let key = self.access_token_secret.as_ref();

        // println!("key: {:?}", key);

        let mut validator = Validation::default();

        validator.set_audience(&["session.plexo.app"]);

        let token_data = decode::<PlexoAuthTokenClaims>(token, &DecodingKey::from_secret(key), &validator);

        // println!("token_data: {:?}", token_data);

        let token_data = token_data.unwrap();

        Ok(token_data.claims)
    }

    // pub fn decode_access_token(&self, token: &str) -> Result<PlexoAuthTokenClaims, Error> {
    //     let token_data = decode::<PlexoAuthTokenClaims>(
    //         token,
    //         &DecodingKey::from_secret(self.access_token_secret.as_ref()),
    //         &jsonwebtoken::Validation::default(),
    //     )?;

    //     Ok(token_data.claims)
    // }

    // pub fn decode_refresh_token(&self, token: &str) -> Result<PlexoAuthTokenClaims, Error> {
    //     let token_data = decode::<PlexoAuthTokenClaims>(
    //         token,
    //         &DecodingKey::from_secret(self.refresh_token_secret.as_ref()),
    //         &jsonwebtoken::Validation::default(),
    //     )?;

    //     Ok(token_data.claims)
    // }

    // pub fn refresh_access_token(
    //     &self,
    //     access_token: &str,
    //     refresh_token: &str,
    // ) -> Result<String, jsonwebtoken::errors::Error> {
    //     let mut claims_access_token = self.decode_access_token(access_token)?;
    //     let _claims_refresh_token = self.decode_refresh_token(refresh_token)?;

    //     claims_access_token.exp += 1000; // TODO

    //     let token = encode(
    //         &Header::default(),
    //         &claims_access_token,
    //         &EncodingKey::from_secret(self.access_token_secret.as_ref()),
    //     )?;

    //     Ok(token)
    // }
}
