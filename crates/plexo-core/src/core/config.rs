use std::env::var;

use lazy_static::lazy_static;

lazy_static! {
    pub static ref HOST: String = var("HOST").unwrap_or("0.0.0.0".into());
    pub static ref PORT: String = var("PORT").unwrap_or("8080".into());
    pub static ref URL: String = var("URL").unwrap_or(format!("{}:{}", *HOST, *PORT));
    pub static ref SCHEMA: String = var("SCHEMA").unwrap_or("http".into());
    pub static ref DOMAIN: String = var("DOMAIN").unwrap_or(format!("{}://{}", *SCHEMA, *URL));
    //
    pub static ref COOKIE_SESSION_DOMAIN: String = var("COOKIE_SESSION_DOMAIN").unwrap_or(format!(".{}", *HOST));
    pub static ref COOKIE_SESSION_NAME: String = var("COOKIE_SESSION_NAME").unwrap_or("plexo-session-token".into());
    pub static ref COOKIE_SESSION_SECURE: String = var("COOKIE_SESSION_SECURE").unwrap_or("false".into());
    pub static ref COOKIE_SESSION_SAME_SITE: String = var("COOKIE_SESSION_SAME_SITE").unwrap_or("none".into());
    //
    pub static ref DATABASE_URL: String = var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    pub static ref GITHUB_CLIENT_ID: Option<String> = var("GITHUB_CLIENT_ID").ok();
    pub static ref GITHUB_CLIENT_SECRET: Option<String> = var("GITHUB_CLIENT_SECRET").ok();
    pub static ref GITHUB_REDIRECT_URL: String = var("GITHUB_REDIRECT_URL").unwrap_or(format!("{}/auth/github/callback", *DOMAIN));
    //
    pub static ref LLM_API_KEY: String = var("OPENAI_API_KEY").expect("OPENAI_API_KEY environment variable not set");
    pub static ref LLM_MODEL_NAME: String = var("LLM_MODEL_NAME").unwrap_or("gpt-3.5-turbo".into());
    //
    pub static ref ADMIN_EMAIL: String = var("ADMIN_EMAIL").unwrap_or("admin@plexo.app".into());
    pub static ref ADMIN_PASSWORD: String = var("ADMIN_PASSWORD").unwrap_or("admin".into());
    pub static ref ADMIN_NAME: String = var("ADMIN_NAME").unwrap_or("Admin".into());
    pub static ref ADMIN_PHOTO_URL: String = var("ADMIN_PHOTO_URL").unwrap_or("https://unavatar.io/plexo.app".into());
    //
    pub static ref ORGANIZATION_NAME: String = var("ORGANIZATION_NAME").unwrap_or("Plexo".into());
    pub static ref ORGANIZATION_EMAIL: String = var("ORGANIZATION_EMAIL").unwrap_or("admin@plexo.app".into());
    pub static ref ORGANIZATION_PHOTO_URL: String = var("ORGANIZATION_PHOTO_URL").unwrap_or("https://unavatar.io/plexo.app".into());
    pub static ref ORGANIZATION_HUB_ID: Option<String> = var("ORGANIZATION_HUB_ID").ok();
    pub static ref ORGANIZATION_PLAN_ID: Option<String> = var("ORGANIZATION_PLAN_ID").ok();
    pub static ref ORGANIZATION_URL: String = var("ORGANIZATION_URL").unwrap_or(format!("https://{}.plexo.app", *ORGANIZATION_NAME));
    //
    pub static ref JWT_ACCESS_TOKEN_SECRET: String = var("JWT_ACCESS_TOKEN_SECRET").unwrap_or("secret".into());
    pub static ref JWT_REFRESH_TOKEN_SECRET: String = var("JWT_REFRESH_TOKEN_SECRET").unwrap_or("secret".into());
    //
    // pub static ref STATIC_PAGE_ENABLED: bool = var("STATIC_PAGE_ENABLED").unwrap_or("false".into()).to_lowercase() == "true";
    //
    pub static ref TRACING_LEVEL: String = var("TRACING_LEVEL").unwrap_or("info".into());
    //
    pub static ref SMTP_HOST: Option<String> = var("SMTP_HOST").ok();
    pub static ref SMTP_PORT: String = var("SMTP_PORT").unwrap_or("25".into());
    pub static ref SMTP_USERNAME: Option<String> = var("SMTP_USERNAME").ok();
    pub static ref SMTP_PASSWORD: Option<String> = var("SMTP_PASSWORD").ok();
}
