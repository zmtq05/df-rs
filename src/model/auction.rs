use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

use super::item::ItemExt;

// TODO: use DateTime
// format: "yyyy-MM-dd hh:mm:ss"
time::serde::format_description!(
    auction_date_format,
    PrimitiveDateTime,
    "[year]-[month]-[day] [hour]:[minute]:[second]"
);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionInfo {
    #[serde(rename = "auctionNo")]
    pub no: u32,
    #[serde(with = "auction_date_format")]
    pub reg_date: PrimitiveDateTime,
    #[serde(with = "auction_date_format")]
    pub expire_date: PrimitiveDateTime,
    #[serde(flatten)]
    pub item: ItemExt,
    pub adventure_fame: u16,
    pub count: u32,
    pub current_price: u32,
    pub unit_price: u32,
    pub average_price: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SoldAuctionInfo {
    #[serde(with = "auction_date_format")]
    pub sold_date: PrimitiveDateTime,
    #[serde(flatten)]
    pub item: ItemExt,
    pub count: u32,
    pub price: u32,
    pub unit_price: u32,
}
