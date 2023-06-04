use std::fmt::Display;

use serde::Deserialize;

#[derive(Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Server {
    All,
    Anton,
    Bakal,
    Cain,
    Casillas,
    Diregie,
    Hilder,
    Prey,
    Siroco,
}

impl Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Server::All => write!(f, "all"),
            Server::Anton => write!(f, "anton"),
            Server::Bakal => write!(f, "bakal"),
            Server::Cain => write!(f, "cain"),
            Server::Casillas => write!(f, "casillas"),
            Server::Diregie => write!(f, "diregie"),
            Server::Hilder => write!(f, "hilder"),
            Server::Prey => write!(f, "prey"),
            Server::Siroco => write!(f, "siroco"),
        }
    }
}

#[derive(Clone, Deserialize)]
pub struct Slot {
    #[serde(rename = "slotId")]
    pub id: String,
    #[serde(rename = "slotName")]
    pub name: String,
}

mod character;
pub use character::*;

mod item;
pub use item::*;

mod auction;
pub use auction::*;

mod serde_helper;
