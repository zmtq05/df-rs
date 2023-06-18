use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};

use super::serde_helper;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Item {
    #[serde(rename = "itemId")]
    pub id: String,
    #[serde(rename = "itemName")]
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemWithRarity {
    #[serde(rename = "itemId")]
    pub id: String,
    #[serde(rename = "itemName")]
    pub name: String,
    #[serde(rename = "itemRarity")]
    pub rarity: ItemRarity,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, DeserializeFromStr, SerializeDisplay,
)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemType {
    #[serde(rename = "itemTypeId")]
    pub id: String,
    #[serde(rename = "itemType", default)]
    pub name: String,
    #[serde(flatten)]
    pub detail: ItemTypeDetail,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ItemTypeDetail {
    #[serde(rename = "itemTypeDetailId")]
    pub id: String,
    #[serde(rename = "itemTypeDetail", default)]
    pub name: String,
}

// ------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
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

    #[serde(rename = "itemStatus", default)]
    pub status: super::Status,

    pub grow_info: Option<GrowInfo>,

    #[serde(rename = "hashtag")]
    pub hashtags: Option<Vec<String>>,

    #[serde(rename = "itemReinforceSkill")]
    pub reinforce_skill: Option<Vec<ReinforceSkill>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Explain {
    #[serde(rename = "itemExplain")]
    pub value: String,
    #[serde(rename = "itemExplainDetail")]
    pub detail: String,
}

// ------------------------------------ ObtainInfo START

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ObtainInfo {
    pub dungeon: Option<Vec<DungeonObtainInfo>>,

    #[serde(deserialize_with = "serde_helper::flatten_shop_obtain_info")]
    pub shop: Option<Vec<ShopObtainInfo>>,

    pub etc: Option<Vec<EtcObtainInfo>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DungeonObtainInfo {
    // dungeon type
    //
    // 일반 던전, 상급 던전, 레기온, 레이드
    pub r#type: String,

    #[serde(deserialize_with = "serde_helper::flatten_rows", rename = "dungeon")]
    pub dungeons: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ShopObtainInfo {
    // ex) 모험단 상점, NPC 린지 로섬, ...
    pub name: String,
    // ex) ["105레벨 에픽 장비 선택 상자"]
    #[serde(default)]
    pub details: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EtcObtainInfo {
    pub name: String,
    pub rows: Vec<EtcObtainInfoRow>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EtcObtainInfoRow {
    pub name: String,
    pub details: Option<Vec<String>>,
}

// ------------------------------------ ObtainInfo END

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Set {
    #[serde(rename = "setItemId")]
    pub id: String,
    #[serde(rename = "setItemName")]
    pub name: String,
}

// ------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReinforceSkill {
    #[serde(flatten)]
    pub job: super::character::Job,
    pub skills: Vec<ReinforceSkillInfo>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReinforceSkillInfo {
    #[serde(rename = "skillId")]
    pub id: String,
    pub name: String,
    #[serde(deserialize_with = "serde_helper::str_as_u8")]
    pub value: u8,
}

// ------------------------------------

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GrowInfo {
    pub transfer: Option<bool>,
    pub total: GrowTotal,
    pub options: Vec<GrowOption>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GrowTotal {
    pub damage: i32,
    pub buff: i32,
    pub level: i32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GrowOptionDefault {
    pub damage: i32,
    pub buff: i32,
}
