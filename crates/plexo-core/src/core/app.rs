use std::sync::Arc;

use plexo_sdk::backend::{
    engine::{SDKConfig, SDKEngine},
    loaders::SDKLoaders,
};

use crate::{auth::engine::AuthEngine, errors::app::PlexoAppError};

use super::config::{
    GITHUB_CLIENT_ID, GITHUB_CLIENT_SECRET, GITHUB_REDIRECT_URL, JWT_ACCESS_TOKEN_SECRET, SMTP_HOST, SMTP_PASSWORD, SMTP_PORT,
    SMTP_USERNAME,
};

use tracing::info;

use lettre::{message::header::ContentType, transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};

#[derive(Clone)]
pub struct Core {
    pub engine: SDKEngine,
    pub auth: AuthEngine,
    pub loaders: Arc<SDKLoaders>,
    // pub emitters: Arc<Emitters>,
    mail_client: Option<SmtpTransport>,
}

pub async fn new_core_from_env() -> Result<Core, PlexoAppError> {
    let engine = SDKEngine::new(SDKConfig::from_env()).await?;

    match engine.migrate().await {
        Ok(_) => info!("database migration successful"),
        Err(err) => info!("database migration failed: {:?}", err),
    }

    let arc_engine = Arc::new(engine.clone());

    let loaders = Arc::new(SDKLoaders::new(arc_engine));

    let auth = AuthEngine::new(
        (*JWT_ACCESS_TOKEN_SECRET).to_string(),
        (*JWT_ACCESS_TOKEN_SECRET).to_string(),
        (*GITHUB_CLIENT_ID).to_owned(),
        (*GITHUB_CLIENT_SECRET).to_owned(),
        Some((*GITHUB_REDIRECT_URL).to_owned()),
    );

    let mail_client = match (
        (*SMTP_HOST).to_owned(),
        (*SMTP_PORT).to_owned(),
        (*SMTP_USERNAME).to_owned(),
        (*SMTP_PASSWORD).to_owned(),
    ) {
        (Some(smtp_host), _smtp_port, Some(smtp_username), Some(smtp_password)) => {
            let credentials = Credentials::new(smtp_username, smtp_password);

            info!("SMTP configuration values are present");

            Some(
                SmtpTransport::relay(smtp_host.as_str())
                    .unwrap()
                    .credentials(credentials)
                    .build(),
            )
        }
        _ => {
            info!("SMTP configuration values are missing");
            None
        }
    };

    Ok(Core {
        engine,
        auth,
        loaders,
        mail_client,
    })
}

impl Core {
    pub fn send_email(&self, from: &str, to: &str, subject: &str, html: &str) -> Result<(), PlexoAppError> {
        let email = Message::builder()
            .from(from.parse().unwrap())
            .to(to.parse().unwrap())
            .subject(subject)
            .header(ContentType::TEXT_HTML)
            .body(html.to_string())
            .unwrap();

        match &self.mail_client {
            Some(client) => {
                let response = client.send(&email).unwrap();
                info!("email sent: {:?}", response);
            } // client.send(mail)?,
            None => info!("no mail client configured, skipping email send"),
        };

        Ok(())
    }
}
