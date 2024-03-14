use crate::core::config::COOKIE_SESSION_NAME;

use super::resources::PlexoAuthToken;
use cookie::Cookie;
use poem::http::HeaderMap;

pub const GITHUB_USER_API: &str = "https://api.github.com/user";

pub fn get_token_from_headers(headers: &HeaderMap) -> Option<PlexoAuthToken> {
    headers
        .get("Authorization")
        .and_then(|value| value.to_str().map(|s| PlexoAuthToken(s.to_string())).ok())
}

pub fn get_token_from_cookie(headers: &HeaderMap) -> Option<PlexoAuthToken> {
    let raw_cookie = headers.get("Cookie").and_then(|c| c.to_str().ok())?;

    get_token_from_raw_cookie(raw_cookie)
}

pub fn get_token_from_raw_cookie(raw_cookie: &str) -> Option<PlexoAuthToken> {
    for cookie in Cookie::split_parse(raw_cookie) {
        let Ok(cookie) = cookie else {
            println!("Error parsing cookie");
            continue;
        };

        if cookie.name() == COOKIE_SESSION_NAME.to_string() {
            return Some(PlexoAuthToken(cookie.value().to_string()));
        }
    }

    None
}
