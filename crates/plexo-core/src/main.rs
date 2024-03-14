use std::{error::Error, str::FromStr};

use dotenv::dotenv;
use plexo_core::{
    api::{graphql::schema::GraphQLSchema, openapi::api::PlexoOpenAPI},
    auth::handlers::{email_basic_login_handler, github_callback_handler, github_sign_in_handler, logout_handler},
    core::{
        app::new_core_from_env,
        config::{DOMAIN, TRACING_LEVEL, URL},
    },
    handlers::{graphiq_handler, graphql_handler, version_handler, ws_switch_handler},
};
use poem::{get, listener::TcpListener, middleware::Cors, post, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
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

    let org = core.prelude().await?;

    info!("welcome to {:?}", org.name);

    let graphql_schema = core.graphql_api_schema();

    let api_prefix = "/v1/api";

    let openapi_server = format!("{}{}", *DOMAIN, api_prefix);

    let api_service = OpenApiService::new(PlexoOpenAPI::new(core.clone()), "Plexo Open API", "1.0").server(openapi_server);

    let spec_json_handler = api_service.spec_endpoint();
    let spec_yaml_handler = api_service.spec_endpoint_yaml();

    let swagger_ui = api_service.swagger_ui();

    let app = Route::new()
        .nest(api_prefix, api_service)
        .nest("/swagger", swagger_ui)
        .at("/openapi.json", get(spec_json_handler))
        .at("/openapi.yaml", get(spec_yaml_handler))
        // .nest("/", static_page)
        // Non authenticated routes
        .at("/auth/email/login", post(email_basic_login_handler))
        // .at("/auth/email/register", post(email_basic_register_handler))
        //
        .at("/auth/github", get(github_sign_in_handler))
        .at("/auth/github/callback", get(github_callback_handler))
        //
        .at("/auth/logout", get(logout_handler))
        //
        .at("/version", get(version_handler))
        //
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
