// use crate::auth::resources::PlexoAuthToken;
// use crate::{auth::jwt::PlexoAuthTokenClaims, core::app::Core};
// use poem::Request;
use poem_openapi::auth::ApiKey;
use poem_openapi::SecurityScheme;
// pub struct PlexoAPIKeyAuthorization(pub PlexoAuthTokenClaims);

// #[poem_openapi::__private::poem::async_trait]
// impl<'a> poem_openapi::ApiExtractor<'a> for PlexoAPIKeyAuthorization {
//     const TYPES: &'static [poem_openapi::ApiExtractorType] = &[poem_openapi::ApiExtractorType::SecurityScheme];

//     const PARAM_IS_REQUIRED: bool = true;

//     type ParamType = ();
//     type ParamRawType = ();

//     fn register(registry: &mut poem_openapi::registry::Registry) {
//         registry.create_security_scheme(
//             "PlexoAPIKeyAuthorization",
//             poem_openapi::registry::MetaSecurityScheme {
//                 ty: "apiKey",
//                 description: ::std::option::Option::Some("Plexo API Key Authorization"),
//                 name: ::std::option::Option::Some("Authorization"),
//                 key_in: ::std::option::Option::Some("header"),
//                 scheme: ::std::option::Option::None,
//                 bearer_format: ::std::option::Option::None,
//                 flows: ::std::option::Option::None,
//                 openid_connect_url: ::std::option::Option::None,
//             },
//         );
//     }

//     fn security_schemes() -> ::std::vec::Vec<&'static str> {
//         ::std::vec!["MyApiKeyAuthorization"]
//     }

//     async fn from_request(
//         req: &'a poem_openapi::__private::poem::Request,
//         _body: &mut poem_openapi::__private::poem::RequestBody,
//         _param_opts: poem_openapi::ExtractParamOptions<Self::ParamType>,
//     ) -> poem_openapi::__private::poem::Result<Self> {
//         let query = req.extensions().get::<poem_openapi::__private::UrlQuery>().unwrap();
//         let output = poem_openapi::__private::CheckerReturn::from(
//             api_checker(
//                 req,
//                 <poem_openapi::auth::ApiKey as poem_openapi::auth::ApiKeyAuthorization>::from_request(
//                     req,
//                     query,
//                     "Authorization",
//                     poem_openapi::registry::MetaParamIn::Header,
//                 )?,
//             )
//             .await,
//         )
//         .into_result()?;
//         ::std::result::Result::Ok(Self(output))
//     }
// }

// async fn api_checker(req: &Request, api_key: ApiKey) -> Option<PlexoAuthTokenClaims> {
//     let core = req.data::<Core>().unwrap();
//     let auth_token = &PlexoAuthToken(api_key.key);

//     core.auth.extract_claims(auth_token).ok()
// }

#[derive(SecurityScheme)]
#[oai(ty = "api_key", key_name = "Authorization", key_in = "header")]
pub struct PlexoAPIKeyAuthorization(ApiKey);
