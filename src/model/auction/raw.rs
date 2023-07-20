// Last Update: 2023-07-20 23:06

use serde::{Deserialize, Serialize};
use time::{macros::offset, OffsetDateTime, PrimitiveDateTime};

use crate::model::{
    Amplification, ItemAvailableLevel, ItemExt, ItemRarity, ItemType, ItemTypeDetail, Refine,
    Reinforce,
};

/// 15. 경매장 등록 아이템 검색
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionItem {
    pub auction_no: u32,

    pub reg_date: DateTime,
    pub expire_date: DateTime,

    pub item_id: String,
    pub item_name: String,
    pub item_available_level: ItemAvailableLevel,
    pub item_rarity: ItemRarity,
    pub item_type_id: String,
    pub item_type: String,
    pub item_type_detail_id: String,
    pub item_type_detail: String,
    pub refine: Refine,
    pub reinforce: Reinforce,
    pub amplification_name: Amplification,

    pub adventure_fame: u32,

    pub count: u32,
    pub price: i32,
    pub current_price: u32,
    pub unit_price: u32,
    pub average_price: u32,
}

impl From<AuctionItem> for super::AuctionItem {
    fn from(value: AuctionItem) -> Self {
        let AuctionItem {
            auction_no,
            reg_date,
            expire_date,
            item_id,
            item_name,
            item_available_level,
            item_rarity,
            item_type_id,
            item_type,
            item_type_detail_id,
            item_type_detail,
            refine,
            reinforce,
            amplification_name,
            adventure_fame,
            count,
            price: _,
            current_price,
            unit_price,
            average_price,
        } = value;

        super::AuctionItem {
            no: auction_no,
            reg_date: reg_date.into(),
            expire_date: expire_date.into(),
            item: ItemExt {
                id: item_id,
                name: item_name,
                rarity: item_rarity,
                r#type: ItemType {
                    id: item_type_id,
                    name: item_type,
                    detail: ItemTypeDetail {
                        id: item_type_detail_id,
                        name: item_type_detail,
                    },
                },
                refine,
                reinforce,
                amplification_name,
                available_level: item_available_level,
            },
            adventure_fame,
            count,
            current_price,
            unit_price,
            average_price,
        }
    }
}

#[derive(Deserialize)]
pub struct AuctionSearchResult {
    pub rows: Vec<AuctionItem>,
}

/// 17. 경매장 시세 검색 (판매된 아이템)
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuctionSoldItem {
    pub sold_date: DateTime,

    pub item_id: String,
    pub item_name: String,
    pub item_available_level: ItemAvailableLevel,
    pub item_rarity: ItemRarity,
    pub item_type_id: String,
    pub item_type: String,
    pub item_type_detail_id: String,
    pub item_type_detail: String,
    pub refine: Refine,
    pub reinforce: Reinforce,
    pub amplification_name: Amplification,

    pub count: u32,
    pub price: u32,
    pub unit_price: u32,
}

impl From<AuctionSoldItem> for super::AuctionSoldItem {
    fn from(value: AuctionSoldItem) -> Self {
        let AuctionSoldItem {
            sold_date,
            item_id,
            item_name,
            item_available_level,
            item_rarity,
            item_type_id,
            item_type,
            item_type_detail_id,
            item_type_detail,
            refine,
            reinforce,
            amplification_name,
            count,
            price,
            unit_price,
        } = value;

        super::AuctionSoldItem {
            sold_date: sold_date.into(),
            item: ItemExt {
                id: item_id,
                name: item_name,
                rarity: item_rarity,
                r#type: ItemType {
                    id: item_type_id,
                    name: item_type,
                    detail: ItemTypeDetail {
                        id: item_type_detail_id,
                        name: item_type_detail,
                    },
                },
                refine,
                reinforce,
                amplification_name,
                available_level: item_available_level,
            },
            count,
            price,
            unit_price,
        }
    }
}

time::serde::format_description!(
    auction_date_format,
    PrimitiveDateTime,
    "[year]-[month]-[day] [hour]:[minute]:[second]"
);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DateTime(#[serde(with = "auction_date_format")] PrimitiveDateTime);
impl From<DateTime> for OffsetDateTime {
    fn from(val: DateTime) -> Self {
        val.0.assume_offset(offset!(+9))
    }
}
