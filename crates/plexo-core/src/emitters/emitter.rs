use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::errors::app::PlexoAppError;

pub trait Emitter {
    fn emit<P>(&self, payload: P) -> Result<(), PlexoAppError>
    where
        P: Serialize + Deserialize<'static> + Directional<&'static str> + Definable;
}

pub trait Directional<D>
where
    D: Display + Serialize + Deserialize<'static>,
{
    fn from(&self) -> D;
    fn to(&self) -> &'static [D];
}

pub trait Definable {
    fn subject(&self) -> &str;
    fn html(&self) -> &str;
}
