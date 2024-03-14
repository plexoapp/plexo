// use crate::errors::app::PlexoAppError;

// use super::emitter::{Definable, Directional, Emitter};
// use serde::{Deserialize, Serialize};

// pub struct ResendEmitter {
//     client: (),
// }

// impl ResendEmitter {
//     pub fn new() -> Self {
//         // let api_key = (*RESEND_API_KEY).to_owned().unwrap();
//         // let client = ResendClient::new(api_key);

//         Self { client: () }
//     }
// }

// impl Default for ResendEmitter {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl Emitter for ResendEmitter {
//     fn emit<P>(&self, message: P) -> Result<(), PlexoAppError>
//     where
//         P: Serialize + Deserialize<'static> + Directional<&'static str> + Definable,
//     {
//         let from = message.from();
//         // let from = "onboarding@plexo.app";

//         let to = message.to();
//         // let to = &[""];

//         let subject = message.subject();
//         // "Welcome to Plexo!";

//         let html = message.html();
//         // "<h1>Welcome to Plexo!</h1>";

//         // let mail = Mail::new(from, to, subject, html);

//         // self.client.send(mail).map_err(|err| err.into())
//         Ok(())
//     }
// }
