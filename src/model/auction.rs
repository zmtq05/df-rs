pub(crate) mod raw;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::item::ItemExt;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct AuctionItem {
    pub no: u32,
    pub reg_date: OffsetDateTime,
    pub expire_date: OffsetDateTime,
    pub item: ItemExt,
    pub adventure_fame: u32,
    pub count: u32,
    pub current_price: u32,
    pub unit_price: u32,
    pub average_price: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "typescript", derive(specta::Type))]
#[serde(rename_all = "camelCase")]
pub struct AuctionSoldItem {
    pub sold_date: OffsetDateTime,
    pub item: ItemExt,
    pub count: u32,
    pub price: u32,
    pub unit_price: u32,
}
