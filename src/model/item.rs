use std::{fmt::Display, str::FromStr};

use serde::Deserialize;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use super::serde_helper;

#[derive(Clone, Deserialize)]
pub struct Item {
    #[serde(rename = "itemId")]
    pub id: String,
    #[serde(rename = "itemName")]
    pub name: String,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemWithRarity {
    #[serde(rename = "itemId")]
    pub id: String,
    #[serde(rename = "itemName")]
    pub name: String,
    #[serde(rename = "itemRarity")]
    pub rarity: ItemRarity,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemExt {
    #[serde(rename = "itemId")]
    pub id: String,
    #[serde(rename = "itemName")]
    pub name: String,
    #[serde(rename = "itemRarity")]
    pub rarity: ItemRarity,
    #[serde(flatten)]
    pub r#type: ItemType,
    pub refine: u8,
    pub reinforce: u8,
    pub amplification_name: Option<String>,
    #[serde(rename = "itemAvailableLevel")]
    pub available_level: u8,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchItem {
    #[serde(rename = "itemId")]
    pub id: String,
    #[serde(rename = "itemName")]
    pub name: String,
    #[serde(rename = "itemRarity")]
    pub rarity: ItemRarity,
    #[serde(flatten)]
    pub r#type: ItemType,
    #[serde(rename = "itemAvailableLevel")]
    pub available_level: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, DeserializeFromStr, SerializeDisplay)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    Unique,
    Chronicle,
    Legendary,
    Epic,
    Mythic,
}

impl FromStr for ItemRarity {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "커먼" => Ok(ItemRarity::Common),
            "언커먼" => Ok(ItemRarity::Uncommon),
            "레어" => Ok(ItemRarity::Rare),
            "유니크" => Ok(ItemRarity::Unique),
            "크로니클" => Ok(ItemRarity::Chronicle),
            "레전더리" => Ok(ItemRarity::Legendary),
            "에픽" => Ok(ItemRarity::Epic),
            "신화" => Ok(ItemRarity::Mythic),
            _ => Err(format!("unknown rarity: {}", s)),
        }
    }
}

impl Display for ItemRarity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ItemRarity::Common => "커먼",
            ItemRarity::Uncommon => "언커먼",
            ItemRarity::Rare => "레어",
            ItemRarity::Unique => "유니크",
            ItemRarity::Chronicle => "크로니클",
            ItemRarity::Legendary => "레전더리",
            ItemRarity::Epic => "에픽",
            ItemRarity::Mythic => "신화",
        };

        f.write_str(str)
    }
}

#[derive(Clone, Deserialize)]
pub struct ItemType {
    #[serde(rename = "itemTypeId")]
    pub id: String,
    #[serde(rename = "itemType", default)]
    pub name: String,
    #[serde(flatten)]
    pub detail: ItemTypeDetail,
}

#[derive(Clone, Deserialize)]
pub struct ItemTypeDetail {
    #[serde(rename = "itemTypeDetailId")]
    pub id: String,
    #[serde(rename = "itemTypeDetail", default)]
    pub name: String,
}

// ------------------------------------

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemInfo {
    #[serde(rename = "itemId")]
    pub id: String,

    #[serde(rename = "itemName")]
    pub name: String,

    #[serde(rename = "itemRarity")]
    pub rarity: ItemRarity,

    #[serde(flatten)]
    pub r#type: ItemType,

    #[serde(rename = "itemAvailableLevel")]
    pub available_level: u8,

    #[serde(flatten)]
    pub explain: Explain,

    #[serde(rename = "itemFlavorText")]
    pub flavor_text: String,

    pub obtain_info: ObtainInfo,

    #[serde(flatten)]
    pub set: Option<Set>,

    #[serde(rename = "itemStatus")]
    pub status: super::character::Status,

    pub grow_info: Option<GrowInfo>,

    #[serde(rename = "hashtag")]
    pub hashtags: Option<Vec<String>>,

    #[serde(rename = "itemReinforceSkill")]
    pub reinforce_skill: Option<Vec<ReinforceSkill>>,
}

#[derive(Clone, Deserialize)]
pub struct Explain {
    #[serde(rename = "itemExplain")]
    pub value: String,
    #[serde(rename = "itemExplainDetail")]
    pub detail: String,
}

// ------------------------------------ ObtainInfo START

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObtainInfo {
    pub dungeon: Option<Vec<DungeonObtainInfo>>,

    #[serde(deserialize_with = "serde_helper::flatten_shop_obtain_info")]
    pub shop: Option<Vec<ShopObtainInfo>>,

    pub etc: Option<Vec<EtcObtainInfo>>,
}

#[derive(Clone, Deserialize)]
pub struct DungeonObtainInfo {
    // dungeon type
    //
    // 일반 던전, 상급 던전, 레기온, 레이드
    pub r#type: String,

    #[serde(deserialize_with = "serde_helper::flatten_rows", rename = "dungeon")]
    pub dungeons: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub struct ShopObtainInfo {
    // ex) 모험단 상점, NPC 린지 로섬, ...
    pub name: String,
    // ex) ["105레벨 에픽 장비 선택 상자"]
    pub details: Vec<String>,
}

#[derive(Clone, Deserialize)]
pub struct EtcObtainInfo {
    pub name: String,
    pub rows: Vec<EtcObtainInfoRow>,
}

#[derive(Clone, Deserialize)]
pub struct EtcObtainInfoRow {
    pub name: String,
    pub details: Option<Vec<String>>,
}

// ------------------------------------ ObtainInfo END

#[derive(Clone, Deserialize)]
pub struct Set {
    #[serde(rename = "setItemId")]
    pub id: String,
    #[serde(rename = "setItemName")]
    pub name: String,
}

// ------------------------------------

#[derive(Clone, Deserialize)]
pub struct ReinforceSkill {
    #[serde(flatten)]
    pub job: super::character::Job,
    pub skiils: Vec<ReinforceSkillInfo>,
}

#[derive(Clone, Deserialize)]
pub struct ReinforceSkillInfo {
    #[serde(rename = "skillId")]
    pub id: String,
    pub name: String,
    #[serde(deserialize_with = "serde_helper::str_as_u8")]
    pub value: u8,
}

// ------------------------------------

#[derive(Clone, Deserialize)]
pub struct GrowInfo {
    pub transfer: Option<bool>,
    pub total: GrowTotal,
    pub options: Vec<GrowOption>,
}

#[derive(Clone, Deserialize)]
pub struct GrowTotal {
    pub damage: i32,
    pub buff: i32,
    pub level: i32,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GrowOption {
    pub level: u8,
    pub exp_rate: f32,
    pub damage: i32,
    pub buff: i32,
    pub explain: String,
    pub explain_detail: String,
    pub default: Option<GrowOptionDefault>,
    pub transfer: Option<bool>,
}

#[derive(Clone, Deserialize)]
pub struct GrowOptionDefault {
    pub damage: i32,
    pub buff: i32,
}
