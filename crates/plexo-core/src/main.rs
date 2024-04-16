use dotenv::dotenv;
use plexo_core::{
    api::graphql::schema::GraphQLSchema,
    auth::handlers::{email_basic_login_handler, github_callback_handler, github_sign_in_handler, logout_handler},
    core::{
        app::new_core_from_env,
        config::{DOMAIN, TRACING_LEVEL, URL},
    },
    handlers::{graphiq_handler, graphql_handler, version_handler, ws_switch_handler},
};
use poem::{get, listener::TcpListener, middleware::Cors, post, EndpointExt, Route, Server};
use std::{error::Error, str::FromStr};
use tracing::{info, subscriber::set_global_default, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::from_str((*TRACING_LEVEL).to_uppercase().as_str()).unwrap_or(Level::INFO))
            .finish(),
    )
    .expect("setting default subscriber failed");

    let core = new_core_from_env().await?;
    let engine_version = core.engine.version().unwrap();

    let org = core.prelude().await?;

    info!("welcome to {:?}", org.name);
    info!("version: {}", engine_version);

    let graphql_schema = core.graphql_api_schema();

    let app = Route::new()
        .at("/auth/email/login", post(email_basic_login_handler))
        .at("/auth/github", get(github_sign_in_handler))
        .at("/auth/github/callback", get(github_callback_handler))
        .at("/auth/logout", get(logout_handler))
        .at("/version", get(version_handler))
        .at("/playground", get(graphiq_handler))
        .at("/graphql", post(graphql_handler))
        .at("/graphql/ws", get(ws_switch_handler));

    let app = app
        .with(Cors::new().allow_credentials(true))
        .data(graphql_schema)
        .data(core.clone());

    info!("visit GraphQL Playground at {}/playground", *DOMAIN);

    Server::new(TcpListener::bind(URL.to_owned()))
        .run(app)
        .await
        .expect("Fail to start web server");

    Ok(())
}
