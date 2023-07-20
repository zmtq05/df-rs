use serde::{Deserialize, Serialize};

use crate::model::ItemAvailableLevel;

use super::{Explain, ItemRarity, ItemType, ItemTypeDetail};

/// 23. 아이템 검색
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SearchItem {
    pub item_id: String,
    pub item_name: String,
    pub item_rarity: ItemRarity,
    pub item_type_id: String,
    pub item_type: String,
    pub item_type_detail_id: String,
    pub item_type_detail: String,
    pub item_available_level: ItemAvailableLevel,
}

impl From<SearchItem> for super::SearchItem {
    fn from(value: SearchItem) -> Self {
        let SearchItem {
            item_id,
            item_name,
            item_rarity,
            item_type_id,
            item_type,
            item_type_detail_id,
            item_type_detail,
            item_available_level,
        } = value;
        Self {
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
            available_level: item_available_level,
        }
    }
}

/// 24. 아이템 상세 정보 조회
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ItemInfo {
    pub item_id: String,
    pub item_name: String,
    pub item_rarity: ItemRarity,
    pub item_type_id: String,
    pub item_type: String,
    pub item_type_detail_id: String,
    pub item_type_detail: String,
    pub item_available_level: ItemAvailableLevel,
    pub item_explain: String,
    pub item_explain_detail: String,
    pub item_flavor_text: String,
    pub obtain_info: Option<ObtainInfo>,
    pub set_item_id: Option<String>,
    pub set_item_name: Option<String>,
    #[serde(default)]
    pub item_status: Vec<Status>,
    #[serde(default)]
    pub item_reinforce_skill: Vec<ReinforceSkill>,
    #[serde(default)]
    pub creature_info: Option<CreatureInfo>,
    #[serde(default)]
    pub card_info: Option<CardInfo>,
    #[serde(default)]
    pub grow_info: Option<GrowInfo>,
    #[serde(default)]
    pub item_buff: Option<ItemBuff>,
    #[serde(default)]
    pub ispins_info: Option<FusionInfo>,
    #[serde(default)]
    pub machine_revolution_info: Option<FusionInfo>,
    #[serde(default)]
    pub dimension_cloister_info: Option<FusionInfo>,
    #[serde(default)]
    pub bakal_info: Option<FusionInfo>,
    #[serde(default)]
    pub talisman_info: Option<TalismanInfo>,
    #[serde(default)]
    pub rune_info: Option<RuneInfo>,
    #[serde(default)]
    pub hashtag: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObtainInfo {
    pub dungeon: Vec<DungeonObtainInfo>,
    pub shop: Vec<ShopObtainInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonObtainInfo {
    pub r#type: String,
    pub rows: Vec<DungeonObtainInfoRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DungeonObtainInfoRow {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopObtainInfo {
    pub rows: Vec<ShopObtainInfoRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopObtainInfoRow {
    pub name: String,
    pub details: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Status {
    pub name: String,
    pub value: ItemStatusValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ItemStatusValue {
    String(String),
    Number(u32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GrowInfo {
    pub total: GrowInfoTotal,
    pub options: Vec<GrowInfoOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GrowInfoTotal {
    pub damage: u32,
    pub buff: u32,
    pub level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GrowInfoOption {
    pub level: u32,
    pub exp_rate: u32,
    pub explain: String,
    pub explain_detail: String,
    pub damage: u32,
    pub buff: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReinforceSkill {
    pub job_name: String,
    pub job_id: String, // null when job_name is "공용"
    #[serde(default)]
    pub skills: Vec<ReinforceSkillRow>,
    #[serde(default)]
    pub level_range: Vec<ReinforceSkillLevelRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReinforceSkillRow {
    pub skill_id: String,
    pub name: String,
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CardInfo {
    pub slots: Vec<CardInfoSlot>,
    pub enchant: Vec<CardInfoEnchant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CardInfoSlot {
    pub slot_id: String,
    pub slot_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CardInfoEnchant {
    pub reinforce_skill: Vec<CardInfoReinforceSkill>,
    pub status: Vec<Status>,
    pub upgrade: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CardInfoReinforceSkill {
    pub job_id: String,
    pub job_name: String,
    pub skills: Vec<CardInfoReinforceSkillRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CardInfoReinforceSkillRow {
    pub skill_id: String,
    pub name: String,
    pub value: String, // NOTE: Why string?
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreatureInfo {
    pub skill: CreatureSkill,
    pub overskill: Option<CreatureOverSkill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreatureSkill {
    pub name: String,
    pub description: String,
    pub cooldown_time: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CreatureOverSkill {
    pub name: String,
    pub description: String,
    pub cooldown_time: u32,
    pub level: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ItemBuff {
    pub explain: String,
    pub explain_detail: String,
    pub reinforce_skill: Vec<ReinforceSkill>, // TODO: Unknown
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FusionInfo {
    pub options: Vec<FusionInfoOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FusionInfoOption {
    #[serde(default)]
    pub damage: u32,
    #[serde(default)]
    pub buff: u32,
    pub explain: String,
    pub explain_detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReinforceSkillLevelRange {
    pub min_level: u32,
    pub max_level: u32,
    pub value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TalismanInfo {
    pub skill_id: String,
    pub skill_name: String,
    pub explain: String,
    pub explain_detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuneInfo {
    pub skill_id: String,
    pub skill_name: String,
    pub rune_type: String,
}

/// 25. 아이템 상점 판매 정보 조회
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShopItem {
    pub item_id: String,
    pub item_name: String,
    pub item_grade_name: String,
    pub item_grade_value: u32,
    pub item_status: Vec<Status>,
}

/// 28. 세트 아이템 검색
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SearchSetItem {
    pub set_item_id: String,
    pub set_item_name: String,
}

/// 29. 세트 아이템 상세 정보 조회
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetItemInfo {
    pub set_item_id: String,
    pub set_item_name: String,
    pub set_items: Vec<SetItemInfoItem>,
    pub set_item_option: Vec<SetItemInfoOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetItemInfoItem {
    pub slot_id: String,
    pub slot_name: String,
    pub item_id: Option<String>,
    pub item_name: String,
    pub item_rarity: ItemRarity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SetItemInfoOption {
    pub option_no: u8, // {n} 세트 효과
    pub status: Vec<Status>,
}

/* impl From<ItemInfo> for super::ItemInfo {
    fn from(value: ItemInfo) -> Self {
        let ItemInfo {
            item_id,
            item_name,
            item_rarity,
            item_type_id,
            item_type,
            item_type_detail_id,
            item_type_detail,
            item_available_level,
            item_explain,
            item_explain_detail,
            item_flavor_text,
            obtain_info,
            set_item_id,
            set_item_name,
            item_status,
            item_reinforce_skill,
            creature_info,
            card_info,
            grow_info,
            item_buff,
            ispins_info,
            machine_revolution_info,
            dimension_cloister_info,
            bakal_info,
            talisman_info,
            rune_info,
            hashtag,
        } = value;

        super::ItemInfo {
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
            available_level: item_available_level,
            explain: Explain {
                value: item_explain,
                detail: item_explain_detail,
            },
            flavor_text: item_flavor_text,
            obtain_info: obtain_info.map(|obtain_info| super::ObtainInfo {
                dungeon: obtain_info
                    .dungeon
                    .into_iter()
                    .map(|dungeon| DungeonObtainInfo {
                        r#type: dungeon.r#type,
                        rows: dungeon
                            .rows
                            .into_iter()
                            .map(|row| DungeonObtainInfoRow { name: row.name })
                            .collect(),
                    })
                    .collect(),
                shop: obtain_info
                    .shop
                    .into_iter()
                    .map(|shop| ShopObtainInfo {
                        rows: shop
                            .rows
                            .into_iter()
                            .map(|row| ShopObtainInfoRow {
                                name: row.name,
                                details: row.details,
                            })
                            .collect(),
                    })
                    .collect(),
            }),
        }
    }
} */
