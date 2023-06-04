use serde::Deserialize;

use super::item::ItemExt;

// TODO: use DateTime
// format: "yyyy-MM-dd hh:mm:ss"
type Date = String;

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionInfo {
    #[serde(rename = "auctionNo")]
    pub no: u32,
    pub reg_date: Date,
    pub expire_date: Date,
    #[serde(flatten)]
    pub item: ItemExt,
    pub adventure_fame: u16,
    pub count: u32,
    pub current_price: u32,
    pub unit_price: u32,
    pub average_price: u32,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SoldAuctionInfo {
    pub sold_date: Date,
    #[serde(flatten)]
    pub item: ItemExt,
    pub count: u32,
    pub price: u32,
    pub unit_price: u32,
}
