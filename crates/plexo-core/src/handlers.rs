use async_graphql::{
    http::{GraphiQLSource, ALL_WEBSOCKET_PROTOCOLS},
    Data, Schema,
};

use async_graphql_poem::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use reqwest::StatusCode;

use poem::{
    handler,
    http::HeaderMap,
    web::{websocket::WebSocket, Data as PoemData, Html},
    IntoResponse,
};

use crate::{
    api::graphql::schema::{MutationRoot, QueryRoot, SubscriptionRoot},
    auth::{
        commons::{get_token_from_cookie, get_token_from_headers},
        resources::PlexoAuthToken,
    },
    core::config::DOMAIN,
};

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
#[handler]
pub async fn graphiq_handler() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint(format!("{}/graphql", *DOMAIN).as_str())
            .subscription_endpoint(format!("{}/graphql/ws", DOMAIN.replace("http", "ws")).as_str())
            .finish(),
    )
}

#[handler]
pub async fn graphql_handler(
    schema: PoemData<&Schema<QueryRoot, MutationRoot, SubscriptionRoot>>,
    headers: &HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.0;

    if let Some(token) = get_token_from_headers(headers) {
        req = req.data(token);
    }

    if let Some(token) = get_token_from_cookie(headers) {
        req = req.data(token);
    }

    schema.execute(req).await.into()
}

#[handler]
pub async fn ws_switch_handler(
    schema: PoemData<&Schema<QueryRoot, MutationRoot, SubscriptionRoot>>,
    headers: &HeaderMap,
    protocol: GraphQLProtocol,
    websocket: WebSocket,
) -> impl IntoResponse {
    let schema = schema.0.clone();
    let mut token: Option<PlexoAuthToken> = None;

    if let Some(header_token) = get_token_from_headers(headers) {
        token = Some(header_token);
    }

    if let Some(cookie_token) = get_token_from_cookie(headers) {
        token = Some(cookie_token);
    }

    match token {
        Some(token) => {
            let mut data = Data::default();
            data.insert(token);

            websocket
                .protocols(ALL_WEBSOCKET_PROTOCOLS)
                .on_upgrade(move |stream| {
                    GraphQLWebSocket::new(stream, schema, protocol)
                        // .on_connection_init(on_connection_init)
                        .with_data(data)
                        .serve()
                })
                .into_response()
        }
        None => poem::Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("Authorization token is required"),
    }
}

// pub async fn on_connection_init(value: Value) -> async_graphql::Result<Data> {
//     match &value {
//         Value::Object(map) => {
//             println!("{:?}", map);

//             if let Some(Value::String(token)) = map.get("Authorization") {
//                 let mut data = Data::default();
//                 data.insert(PlexoAuthToken(token.to_string()));

//                 Ok(data)
//             } else {
//                 Err("Authorization token is required".into())
//             }
//         }
//         _ => Err("Authorization token is required".into()),
//     }
// }

#[handler]
pub async fn version_handler() -> impl IntoResponse {
    VERSION.unwrap_or("No version found")
}
